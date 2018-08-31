pub mod atlas;
mod tex_key;
mod paint_sys;
pub mod frame_sets;

pub use self::tex_key::{TextureKey, get_asset_by_name, ASSET_NAME_MAP};
pub use self::paint_sys::*;

use math_util;
use glutin::GlWindow;
use gfx_window_glutin;
use camera::Camera;
use self::atlas::*;
use std::default::Default;
use gfx::{IndexBuffer, Slice, self};
use gfx::handle::RenderTargetView;
use gfx::buffer::Role;
use gfx_device_gl::Factory;
use gfx::memory::{Usage, Bind};
use gfx_device_gl::{Resources, CommandBuffer, Device};
use asset_loader;

pub const V_BUF_SIZE: usize = 262144;
const CLEAR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

/// Vertex buffers that exist on the GPU
pub enum BufferType {
    Game, UI, Terrain
}

/// Mask preset for alpha blending
fn mask() -> gfx::state::ColorMask {
    gfx::state::ColorMask::all()
}

/// Blend preset for alpha blending
fn blend() -> gfx::state::Blend {
    use gfx::state::{Equation,Factor,BlendValue};
    gfx::state::Blend::new(
        Equation::Add,
        Factor::ZeroPlus(BlendValue::SourceAlpha),
        Factor::OneMinus(BlendValue::SourceAlpha),
    )
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "pos",
        col: [f32; 4] = "col",
        uv: [f32; 2] = "uv",
    }

    constant Transform {
        proj: [[f32; 4];4] = "proj",
        view: [[f32; 4];4] = "view",
    }

    constant Fog {
        fog_center: [f32; 4] = "fog_center",
        fog_color: [f32; 4] = "fog_color",
        fog_density: f32 = "fog_density",
    }

    pipeline pipe {
        v_buf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        fog: gfx::ConstantBuffer<Fog> = "Fog",
        tex: gfx::TextureSampler<[f32; 4]> = "tex",
        out_col: gfx::BlendTarget<ColorFormat> = ("col", super::mask(), super::blend()),
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: [0.0, 0.0, 0.0],
            col: [0.0, 0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        }
    }
}

pub struct RendererSettings {
    pub aspect: f32,
}

impl Default for RendererSettings {
    fn default() -> Self {
        RendererSettings {
            aspect: 16.0/9.0
        }
    }
}

pub struct Renderer {
    #[allow(dead_code)]
    settings: RendererSettings,

    /// Gfx encoder for drawing & updating buffers
    encoder: gfx::Encoder<Resources, CommandBuffer>,

    game_buf_slice: gfx::Slice<Resources>,
    ui_buf_slice: gfx::Slice<Resources>,
    terrain_buf_slice: gfx::Slice<Resources>,

    /// Pipeline data for main render
    game_pipe_data: pipe::Data<Resources>,

    /// Pipeline data for UI render
    ui_pipe_data: pipe::Data<Resources>,

    /// Pipeline data for terrain render
    terrain_pipe_data: pipe::Data<Resources>,

    pso: gfx::pso::PipelineState<Resources, pipe::Meta>,
}

impl Renderer {
    pub fn new(factory: &mut Factory,
               color_view: RenderTargetView<Resources, ColorFormat>,
               depth_view: gfx::handle::DepthStencilView<Resources, DepthFormat>,
               settings: RendererSettings) -> (Renderer, TextureAtlas<TextureKey>) {
        use gfx::{Factory, traits::FactoryExt};

        // Load the texture atlas
        let (atlas, tex_view) = asset_loader::load_assets(factory);
        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(
                gfx::texture::FilterMethod::Scale,
                gfx::texture::WrapMode::Clamp));

        // Create the encoder
        let encoder : gfx::Encoder<_, _> = factory.create_command_buffer().into();

        // Allocate buffers
        let ui_transform_buffer = factory.create_constant_buffer(1);
        let game_transform_buffer = factory.create_constant_buffer(1);
        let terrain_transform_buffer = factory.create_constant_buffer(1);
        let ui_fog_buffer = factory.create_constant_buffer(1);
        let game_fog_buffer = factory.create_constant_buffer(1);
        let terrain_fog_buffer = factory.create_constant_buffer(1);

        let ui_vertex_buffer = factory.create_buffer::<Vertex>(
            V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();
        let game_vertex_buffer = factory.create_buffer::<Vertex>(
            V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();
        let terrain_vertex_buffer = factory.create_buffer::<Vertex>(
            V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();

        // Create the pipeline data
        let game_pipe_data = pipe::Data {
            v_buf: game_vertex_buffer,
            transform: game_transform_buffer,
            fog: game_fog_buffer,
            tex: (tex_view.clone(), sampler.clone()),
            out_col: color_view.clone(),
            out_depth: depth_view.clone(),
        };

        let terrain_pipe_data = pipe::Data {
            v_buf: terrain_vertex_buffer,
            transform: terrain_transform_buffer,
            fog: terrain_fog_buffer,
            tex: (tex_view.clone(), sampler.clone()),
            out_col: color_view.clone(),
            out_depth: depth_view.clone(),
        };

        let ui_pipe_data = pipe::Data {
            v_buf: ui_vertex_buffer,
            transform: ui_transform_buffer,
            fog: ui_fog_buffer,
            tex: (tex_view, sampler),
            out_col: color_view.clone(),
            out_depth: depth_view.clone(),
        };

        // Setup shaders
        let pso = factory.create_pipeline_simple(
            include_bytes!("vert.glsl"),
            include_bytes!("frag.glsl"),
            pipe::new()
        ).unwrap();

        let empty_slice = Slice {
            start: 0,
            end: 0,
            base_vertex: 0,
            instances: None,
            buffer: IndexBuffer::Auto,
        };

        (Renderer {
            settings: settings,
            encoder: encoder,
            game_buf_slice: empty_slice.clone(),
            ui_buf_slice: empty_slice.clone(),
            terrain_buf_slice: empty_slice,
            game_pipe_data: game_pipe_data,
            ui_pipe_data: ui_pipe_data,
            terrain_pipe_data: terrain_pipe_data,
            pso: pso,
        }, atlas)
    }

    /// Render a rect on the horizontal plane
    fn rect(v_buf: &mut [Vertex], tex: &UvRect,
                    x: f32, y: f32, z: f32,
                    w: f32, h: f32,
                    col: [f32; 4]) {
        debug_assert!(v_buf.len() >= 6, "Drawing rect but v_buf < 6 in len");
        v_buf[0] = Vertex {pos: [x, z, y], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [x+w, z, y], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [x+w, z, y+h], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [x, z, y], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [x, z, y+h], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [x+w, z, y+h], col: col, uv: [tex.right, tex.bottom]};
    }

    /// Render an 'upright' rect (i.e. across the z-plane)
    fn rect_upright(v_buf: &mut [Vertex], tex: &UvRect,
            x: f32, y: f32, z: f32,
            w: f32, h: f32,
            col: [f32; 4]) {
        debug_assert!(v_buf.len() >= 6, "Drawing rect but v_buf < 6 in len");
        v_buf[0] = Vertex {pos: [x, z-h, y], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [x+w, z-h, y], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [x+w, z, y], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [x, z-h, y], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [x, z, y], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [x+w, z, y], col: col, uv: [tex.right, tex.bottom]};
    }

    /// Helper function for buffering a rect to a vec
    /// # Params
    /// * `rot` - The rotation for this rect, origin is center of the rect,
    /// anticlockwise radians
    /// # Panics
    /// If v_buf is not at least 6 vertexes long
    fn rect_rot(v_buf: &mut [Vertex], tex: &UvRect,
                x: f32, y: f32, z: f32,
                w: f32, h: f32,
                col: [f32; 4], rot: f32) {
        debug_assert!(v_buf.len() >= 6, "Drawing rect but v_buf < 6 in len");
        // Rotate all the points around the origin.
        let origin = [x + w / 2.0, y + h / 2.0];
        let p0 = math_util::rotate_point([x, y], &origin, rot);
        let p1 = math_util::rotate_point([x+w, y], &origin, rot);
        let p2 = math_util::rotate_point([x+w, y+h], &origin, rot);
        let p3 = math_util::rotate_point([x, y+h], &origin, rot);
        v_buf[0] = Vertex {pos: [p0[0], z, p0[1]], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [p1[0], z, p1[1]], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [p2[0], z, p2[1]], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [p0[0], z, p0[1]], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [p3[0], z, p3[1]], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [p2[0], z, p2[1]], col: col, uv: [tex.right, tex.bottom]};
    }

    pub fn update_window_size(&mut self, window: &GlWindow) {
        // Update the render target size
        gfx_window_glutin::update_views(window, &mut self.ui_pipe_data.out_col,
                                        &mut self.ui_pipe_data.out_depth);
        gfx_window_glutin::update_views(window, &mut self.terrain_pipe_data.out_col,
                                        &mut self.terrain_pipe_data.out_depth);
        gfx_window_glutin::update_views(window, &mut self.game_pipe_data.out_col,
                                        &mut self.game_pipe_data.out_depth);
    }

    /// Clear the screen (AND depth).
    pub fn clear(&mut self) {
        self.encoder.clear(&self.ui_pipe_data.out_col, CLEAR);
        self.encoder.clear(&self.game_pipe_data.out_col, CLEAR);
        self.encoder.clear(&self.terrain_pipe_data.out_col, CLEAR);
        self.encoder.clear_depth(&self.ui_pipe_data.out_depth, 1.0);
        self.encoder.clear_depth(&self.game_pipe_data.out_depth, 1.0);
        self.encoder.clear_depth(&self.terrain_pipe_data.out_depth, 1.0);
    }

    /// Clear only the depth buffers
    pub fn clear_depth(&mut self) {
        self.encoder.clear_depth(&self.ui_pipe_data.out_depth, 1.0);
        self.encoder.clear_depth(&self.game_pipe_data.out_depth, 1.0);
        self.encoder.clear_depth(&self.terrain_pipe_data.out_depth, 1.0);
    }

    /// Update the GPU buffer with CPU data
    /// # Params
    /// * vertex_buffer - The CPU-side vertex buffer
    /// * buffer_type - The buffer to update
    pub fn update_buffer(&mut self, vertex_buffer: &VertexBuffer, buffer_type: BufferType) {
        let buffer = match buffer_type {
            BufferType::Game => &self.game_pipe_data.v_buf,
            BufferType::UI => &self.ui_pipe_data.v_buf,
            BufferType::Terrain => &self.terrain_pipe_data.v_buf,
        };
        let slice = match buffer_type {
            BufferType::Game => &mut self.game_buf_slice,
            BufferType::UI => &mut self.ui_buf_slice,
            BufferType::Terrain => &mut self.terrain_buf_slice,
        };
        self.encoder.update_buffer(
            buffer, &vertex_buffer.v_buf[0..vertex_buffer.size as usize], 0).unwrap();
        *slice = Slice {
            start: 0,
            end: vertex_buffer.size,
            base_vertex: 0,
            instances: None,
            buffer: IndexBuffer::Auto,
        };
    }

    /// Render a vertex buffer.
    /// # Params
    /// * buffer_type - The buffer to render
    pub fn render_buffer(&mut self, camera: &Camera, player_pos: [f32; 3], buffer_type: BufferType) {
        // Create transform buffer
        let transform = match buffer_type {
            BufferType::Game | BufferType::Terrain => Transform {
                proj: camera.gen_persp_proj_mat(),
                view: camera.gen_view_mat(),
            },
            BufferType::UI => Transform {
                proj: camera.gen_ortho_proj_mat(),
                view: Camera::gen_ui_view_mat(),
            },
        };
        let fog = match buffer_type {
            BufferType::Game | BufferType::Terrain => Fog {
                fog_center: [player_pos[0], player_pos[1], player_pos[2], 0.0],
                fog_color: [0.5, 0.5, 0.5, 1.0],
                fog_density: 0.0005,
            },
            BufferType::UI => Fog {
                fog_center: [0.0, 0.0, 0.0, 0.0],
                fog_color: [0.5, 0.5, 0.5, 1.0],
                fog_density: 0.0,
            },
        };
        let fog_buffer = match buffer_type {
            BufferType::Game => &self.game_pipe_data.fog,
            BufferType::UI => &self.ui_pipe_data.fog,
            BufferType::Terrain => &self.terrain_pipe_data.fog,
        };
        let transform_buffer = match buffer_type {
            BufferType::Game => &self.game_pipe_data.transform,
            BufferType::UI => &self.ui_pipe_data.transform,
            BufferType::Terrain => &self.terrain_pipe_data.transform,
        };
        // Update the transform & fog buffer
        self.encoder.update_buffer(transform_buffer, &[transform], 0).unwrap();
        self.encoder.update_buffer(fog_buffer, &[fog], 0).unwrap();

        // Draw the buffer
        let (pipe_data, slice) = match buffer_type {
            BufferType::Game => (&self.game_pipe_data, &self.game_buf_slice),
            BufferType::UI => (&self.ui_pipe_data, &self.ui_buf_slice),
            BufferType::Terrain => (&self.terrain_pipe_data, &self.terrain_buf_slice),
        };

        self.encoder.draw(slice, &self.pso, pipe_data);

    }

    /// Actually execute commands
    pub fn flush(&mut self, device: &mut Device) {
        self.encoder.flush(device); // execute draw commands
    }
}
