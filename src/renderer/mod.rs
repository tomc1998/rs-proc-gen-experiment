mod atlas;
mod tex_key;
mod paint_sys;

pub use self::tex_key::TextureKey;
pub use self::paint_sys::*;

use self::atlas::*;
use std::default::Default;
use gfx::{IndexBuffer, Slice, self};
use gfx::handle::RenderTargetView;
use gfx::buffer::Role;
use gfx_device_gl::Factory;
use gfx::memory::{Usage, Bind};
use gfx_device_gl::{Resources, CommandBuffer, Device};
use std::collections::BTreeMap;
use fpa::*;

pub const V_BUF_SIZE: usize = 262144;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct Camera {
    x: Fx32,
    y: Fx32,
    w: Fx32,
    h: Fx32,
}

impl Camera {
    pub fn new(w: f32, h: f32) -> Camera {
        Camera { x: Fx32::new(0.0), y: Fx32::new(0.0), w: Fx32::new(w), h: Fx32::new(h) }
    }
    pub fn gen_ortho_mat(&self) -> [[f32; 4]; 4] {
        gen_ortho_mat(self.x.to_f32(), (self.x + self.w).to_f32(),
                      self.y.to_f32(), (self.y + self.h).to_f32(),
                      -10000.0, 10000.0)
    }
}

/// State for rendering. Passed to the render function.
fn gen_ortho_mat(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> [[f32; 4]; 4] {
    [[2.0/(r-l),       0.0,        0.0, -(r+l)/(r-l)],
     [0.0,       2.0/(t-b),        0.0, -(t+b)/(t-b)],
     [0.0,             0.0, -2.0/(f-n), -(f+n)/(f-n)],
     [0.0,             0.0,        0.0,          1.0]]
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
    pub window_size: (u32, u32),

    #[allow(dead_code)]
    settings: RendererSettings,

    /// Gfx encoder for drawing & updating buffers
    encoder: gfx::Encoder<Resources, CommandBuffer>,

    /// The colour view to render to
    color_view: RenderTargetView<Resources, ColorFormat>,
    depth_view: gfx::handle::DepthStencilView<Resources, DepthFormat>,

    /// Pipeline data
    data: pipe::Data<Resources>,

    pso: gfx::pso::PipelineState<Resources, pipe::Meta>,

    /// Transform uniform block, to avoid repeated allocations on render() calls
    transform: Transform,
}

impl Renderer {
    pub fn new(factory: &mut Factory,
               color_view: RenderTargetView<Resources, ColorFormat>,
               depth_view: gfx::handle::DepthStencilView<Resources, DepthFormat>,
               w: u32, h: u32,
               settings: RendererSettings) -> (Renderer, TextureAtlas<TextureKey>) {
        use gfx::{Factory, traits::FactoryExt};
        // Load the texture atlas

        // Initialise common frame maps
        let mut human_frame_map = BTreeMap::new();
        human_frame_map.insert(TextureKey::Human00IdleDown,  &[(0, 0)][..]);
        human_frame_map.insert(TextureKey::Human00IdleUp,    &[(0, 1)][..]);
        human_frame_map.insert(TextureKey::Human00IdleRight, &[(0, 2)][..]);
        human_frame_map.insert(TextureKey::Human00IdleLeft,  &[(0, 3)][..]);
        human_frame_map.insert(TextureKey::Human00WalkDown,  &[(0, 0), (1, 0), (2, 0), (3, 0)][..]);
        human_frame_map.insert(TextureKey::Human00WalkUp,    &[(0, 1), (1, 1), (2, 1), (3, 1)][..]);
        human_frame_map.insert(TextureKey::Human00WalkRight, &[(0, 2), (1, 2), (2, 2), (3, 2)][..]);
        human_frame_map.insert(TextureKey::Human00WalkLeft,  &[(0, 3), (1, 3), (2, 3), (3, 3)][..]);
        human_frame_map.insert(TextureKey::Human00AttackDown,  &[(1, 0)][..]);
        human_frame_map.insert(TextureKey::Human00AttackUp,    &[(1, 1)][..]);
        human_frame_map.insert(TextureKey::Human00AttackRight, &[(1, 2)][..]);
        human_frame_map.insert(TextureKey::Human00AttackLeft,  &[(1, 3)][..]);

        let mut slime_frame_map = BTreeMap::new();
        slime_frame_map.insert(TextureKey::Slime00Idle,  &[(0, 0)][..]);
        slime_frame_map.insert(TextureKey::Slime00Charge,  &[(1, 0)][..]);

        let mut slice_frame_map = BTreeMap::new();
        slice_frame_map.insert(TextureKey::Slice00Down, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)][..]);
        slice_frame_map.insert(TextureKey::Slice00Left, &[(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)][..]);
        slice_frame_map.insert(TextureKey::Slice00Up,   &[(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)][..]);
        slice_frame_map.insert(TextureKey::Slice00Right,&[(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)][..]);

        let (atlas, tex_view) = AtlasBuilder::<TextureKey>::new(512, 512)
            .set_font("res/open-sans.ttf",
                      Charset::alpha()
                      .and(Charset::number())
                      .and(Charset::common_punc())
                      .into_iter(),
                      32.0).unwrap()
            .add_tex(TextureKey::White, "res/white.png").unwrap()
            .add_tex(TextureKey::GreenTree00, "res/sprites/green-tree-00.png").unwrap()
            .add_tileset(TextureKey::TilesetGrass, "res/tileset-grass.png", 8, 8).unwrap()
            .add_anim_sprite("res/sprites/human-00.png", human_frame_map.clone(), 8, 8).unwrap()
            .add_anim_sprite("res/sprites/slime-00.png", slime_frame_map.clone(), 8, 8).unwrap()
            .add_anim_sprite("res/sprites/fx/slice-00.png", slice_frame_map.clone(), 16, 16).unwrap()
            .build(factory);
        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(
                gfx::texture::FilterMethod::Scale,
                gfx::texture::WrapMode::Clamp));

        // Create the encoder
        let encoder : gfx::Encoder<_, _> = factory.create_command_buffer().into();

        // Allocate buffers
        let transform_buffer = factory.create_constant_buffer(1);

        let vertex_buffer = factory.create_buffer::<Vertex>(V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();

        let transform = Transform {
            proj: [[0.0; 4]; 4], // Filled in by camera later
            view: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
        };

        // Create the pipeline data
        let data = pipe::Data {
            v_buf: vertex_buffer,
            transform: transform_buffer,
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

        (Renderer {
            window_size: (w, h),
            settings: settings,
            encoder: encoder,
            data: data,
            pso: pso,
            transform: transform,
            color_view: color_view,
            depth_view: depth_view,
        }, atlas)
    }

    /// Helper function for buffering a rect to a vec
    /// # Panics
    /// If v_buf is not at least 6 vertexes long
    fn rect(v_buf: &mut [Vertex], tex: &UvRect, x: f32, y: f32, z: f32, w: f32, h: f32, col: [f32; 4]) {
        debug_assert!(v_buf.len() >= 6, "Drawing rect but v_buf < 6 in len");
        v_buf[0] = Vertex {pos: [x, y, z], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [x+w, y, z], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [x+w, y+h, z], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [x, y, z], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [x, y+h, z], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [x+w, y+h, z], col: col, uv: [tex.right, tex.bottom]};
    }

    /// Actually issue the draw commands to the GPU. This should be called after
    /// the ECS has run. Device can't be sent over threads, so this is the
    /// simplest way to call draw.
    pub fn flush_render(&mut self, device: &mut Device,
                        vertex_buffer: &VertexBuffer,
                        camera: &Camera) {
        // Update the GPU side vertices
        // TODO: if we haven't updated v_buf cpu side we can potentially skip
        // this as an optimisation
        self.encoder.update_buffer(&self.data.v_buf,
                                   &vertex_buffer.v_buf[0..vertex_buffer.size as usize],
                                   0).unwrap();

        let slice = Slice {
            start: 0,
            end: vertex_buffer.size,
            base_vertex: 0,
            instances: None,
            buffer: IndexBuffer::Auto,
        };

        self.transform.proj = camera.gen_ortho_mat();
        self.encoder.update_buffer(&self.data.transform, &[self.transform], 0).unwrap();
        self.encoder.clear(&self.color_view, BLACK);
        self.encoder.clear_depth(&self.depth_view, 1.0);
        self.encoder.draw(&slice, &self.pso, &self.data);
        self.encoder.flush(device); // execute draw commands
    }
}
