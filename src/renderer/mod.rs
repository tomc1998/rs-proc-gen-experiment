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

pub const V_BUF_SIZE: usize = 262144;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct Camera {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Camera {
    pub fn new(w: f32, h: f32) -> Camera {
        Camera { x: 0.0, y: 0.0, w: w, h: h }
    }
    pub fn gen_ortho_mat(&self) -> [[f32; 4]; 4] {
        gen_ortho_mat(self.x, self.x + self.w, self.y, self.y + self.h, -1.0, 1.0)
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
        pos: [f32; 2] = "pos",
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
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: [0.0, 0.0],
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

    /// Pipeline data
    data: pipe::Data<Resources>,

    pso: gfx::pso::PipelineState<Resources, pipe::Meta>,

    /// Transform uniform block, to avoid repeated allocations on render() calls
    transform: Transform,
}

impl Renderer {
    pub fn new(factory: &mut Factory,
               color_view: RenderTargetView<Resources, ColorFormat>,
               w: u32, h: u32,
               settings: RendererSettings) -> (Renderer, TextureAtlas<TextureKey>) {
        use gfx::{Factory, traits::FactoryExt};
        // Load the texture atlas
        let (atlas, tex_view) = AtlasBuilder::<TextureKey>::new(512, 512)
            .set_font("res/open-sans.ttf",
                      Charset::alpha()
                      .and(Charset::number())
                      .and(Charset::common_punc())
                      .into_iter(),
                      32.0).unwrap()
            .add_tex(TextureKey::White, "res/white.png").unwrap()
            .add_tileset(TextureKey::TilesetGrass, "res/tileset-grass.png", 8, 8).unwrap()
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
            proj: gen_ortho_mat(0.0, 800.0, 0.0, 600.0, -1.0, 1.0),
            view: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
        };

        // Create the pipeline data
        let data = pipe::Data {
            v_buf: vertex_buffer,
            transform: transform_buffer,
            tex: (tex_view, sampler),
            out_col: color_view.clone(),
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
        }, atlas)
    }

    /// Helper function for buffering a rect to a vec
    /// # Panics
    /// If v_buf is not at least 6 vertexes long
    fn rect(v_buf: &mut [Vertex], tex: &UvRect, x: f32, y: f32, w: f32, h: f32, col: [f32; 4]) {
        v_buf[0] = Vertex {pos: [x, y], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [x+w, y], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [x+w, y+h], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [x, y], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [x, y+h], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [x+w, y+h], col: col, uv: [tex.right, tex.bottom]};
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
        self.encoder.draw(&slice, &self.pso, &self.data);
        self.encoder.flush(device); // execute draw commands
    }
}
