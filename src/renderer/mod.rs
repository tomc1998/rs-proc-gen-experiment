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
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

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
        proj: [[f32; 4];4] = "u_proj",
        view: [[f32; 4];4] = "u_view",
    }

    pipeline pipe {
        v_buf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
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

    /// Pipeline data for main render
    game_pipe_data: pipe::Data<Resources>,

    /// Pipeline data for UI render
    ui_pipe_data: pipe::Data<Resources>,

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

        let ui_vertex_buffer = factory.create_buffer::<Vertex>(
            V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();
        let game_vertex_buffer = factory.create_buffer::<Vertex>(
            V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();

        // Create the pipeline data
        let game_pipe_data = pipe::Data {
            v_buf: game_vertex_buffer,
            transform: game_transform_buffer,
            tex: (tex_view.clone(), sampler.clone()),
            out_col: color_view.clone(),
            out_depth: depth_view.clone(),
        };

        let ui_pipe_data = pipe::Data {
            v_buf: ui_vertex_buffer,
            transform: ui_transform_buffer,
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

        (Renderer { settings, encoder, game_pipe_data, ui_pipe_data, pso }, atlas)
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
        gfx_window_glutin::update_views(window, &mut self.game_pipe_data.out_col,
                                        &mut self.game_pipe_data.out_depth);
    }

    /// Clear the screen.
    pub fn clear(&mut self) {
        self.encoder.clear(&self.ui_pipe_data.out_col, BLACK);
        self.encoder.clear(&self.game_pipe_data.out_col, BLACK);
        self.encoder.clear_depth(&self.ui_pipe_data.out_depth, 1.0);
        self.encoder.clear_depth(&self.game_pipe_data.out_depth, 1.0);
    }

    /// Actually issue the draw commands to the GPU. This should be called after
    /// the ECS has run. Device can't be sent over threads, so this is the
    /// simplest way to call draw.
    pub fn flush_render(&mut self, device: &mut Device,
                        game_vertex_buffer: &VertexBuffer,
                        ui_vertex_buffer: &VertexBuffer,
                        camera: &Camera) {
        // // Update the GPU side vertices
        // // TODO: if we haven't updated v_buf cpu side we can potentially skip
        // // this as an optimisation
        self.encoder.update_buffer(&self.game_pipe_data.v_buf,
                                   &game_vertex_buffer.v_buf[0..game_vertex_buffer.size as usize],
                                   0).unwrap();
        self.encoder.update_buffer(&self.ui_pipe_data.v_buf,
                                   &ui_vertex_buffer.v_buf[0..ui_vertex_buffer.size as usize],
                                   0).unwrap();

        let slice = Slice {
            start: 0,
            end: game_vertex_buffer.size,
            base_vertex: 0,
            instances: None,
            buffer: IndexBuffer::Auto,
        };
        // Create transform buffer
        let transform = Transform {
            proj: camera.gen_persp_proj_mat(),
            view: camera.gen_view_mat(),
        };
        self.encoder.update_buffer(&self.game_pipe_data.transform, &[transform], 0).unwrap();
        self.encoder.draw(&slice, &self.pso, &self.game_pipe_data);

        // Clear UI depth
        self.encoder.clear_depth(&self.ui_pipe_data.out_depth, 1.0);

        // render UI
        let slice = Slice {
            start: 0,
            end: ui_vertex_buffer.size,
            base_vertex: 0,
            instances: None,
            buffer: IndexBuffer::Auto,
        };
        // Update the view (no camera transform)
        let transform = Transform {
            proj: camera.gen_ortho_proj_mat(),
            view: Camera::gen_ui_view_mat(),
        };

        self.encoder.update_buffer(&self.ui_pipe_data.transform, &[transform], 0).unwrap();
        // No need to use the same depth buffer, UI will always be rendered over
        // the main world
        self.encoder.draw(&slice, &self.pso, &self.ui_pipe_data);

        self.encoder.flush(device); // execute draw commands
    }
}
