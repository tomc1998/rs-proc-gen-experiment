mod atlas;
mod tex_key;

use self::atlas::*;
use self::tex_key::TextureKey;
use std::default::Default;
use gfx;
use gfx::handle::RenderTargetView;
use gfx::buffer::Role;
use gfx_device_gl::Factory;
use gfx::Device;
use gfx::memory::{Usage, Bind};
use gfx_device_gl::{Resources, CommandBuffer};

const V_BUF_SIZE: usize = 2048;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

/// State for rendering. Passed to the render function.
pub struct RenderState {
}

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
    settings: RendererSettings,

    /// Gfx encoder for drawing & updating buffers
    encoder: gfx::Encoder<Resources, CommandBuffer>,

    /// The colour view to render to
    color_view: RenderTargetView<Resources, ColorFormat>,

    /// The main texture atlas
    #[allow(dead_code)]
    atlas: TextureAtlas<TextureKey>,

    /// Pipeline data
    data: pipe::Data<Resources>,

    pso: gfx::pso::PipelineState<Resources, pipe::Meta>,

    /// Transform uniform block, to avoid repeated allocations on render() calls
    transform: Transform,

    /// CPU vertex buf to avoid repeated allocs on render() calls - this will
    /// just be buffered to the GPU buffer each frame
    v_buf: Vec<Vertex>,
    slice: gfx::Slice<Resources>,
}

impl Renderer {
    pub fn new(factory: &mut Factory,
               color_view: RenderTargetView<Resources, ColorFormat>,
               w: u32, h: u32,
               settings: RendererSettings) -> Renderer {
        use gfx::{Factory, traits::FactoryExt};
        // Load the texture atlas
        let (atlas, tex_view) = AtlasBuilder::<TextureKey>::new(512, 512)
            .set_font("res/open-sans.ttf",
                      Charset::alpha()
                      .and(Charset::number())
                      .and(Charset::common_punc())
                      .into_iter(),
                      32.0).unwrap()
            .add_tex(TextureKey::White, "res/white.png", 0.5).unwrap()
            .build(factory);
        let sampler = factory.create_sampler_linear();

        // Create the encoder
        let encoder : gfx::Encoder<_, _> = factory.create_command_buffer().into();

        // Allocate CPU side vertex buffer.
        let mut v_buf = vec![Default::default(); V_BUF_SIZE];
        {
            let white = atlas.rect_for_key(TextureKey::White).unwrap();
            v_buf[0] = Vertex {
                pos: [0.0, 0.0],
                col: [1.0, 1.0, 1.0, 1.0],
                uv: [white.left, white.top],
            };
            v_buf[1] = Vertex {
                pos: [100.0, 0.0],
                col: [1.0, 1.0, 1.0, 1.0],
                uv: [white.right, white.top],
            };
            v_buf[2] = Vertex {
                pos: [100.0, 100.0],
                col: [1.0, 1.0, 1.0, 1.0],
                uv: [white.right, white.bottom],
            };
        }

        // Allocate buffers
        let transform_buffer = factory.create_constant_buffer(1);

        let vertex_buffer = factory.create_buffer::<Vertex>(v_buf.len(), Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();

        // Get a slice for the vertex buffer
        let slice = gfx::Slice::new_match_vertex_buffer(&vertex_buffer);

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

        Renderer {
            window_size: (w, h),
            settings: settings,
            encoder: encoder,
            data: data,
            pso: pso,
            transform: transform,
            v_buf: v_buf,
            atlas: atlas,
            color_view: color_view,
            slice: slice,
        }
    }

    pub fn render<D>(&mut self, device: &mut D, _state: RenderState)
    where D : Device<CommandBuffer=CommandBuffer, Resources=Resources> {
        // Update the GPU side vertices
        // TODO: if we haven't updated v_buf cpu side we can potentially skip
        // this as an optimisation
        self.encoder.update_buffer(&self.data.v_buf, &self.v_buf[..], 0).unwrap();

        let (l, r, t, b) = (0.0, self.window_size.0 as f32, 0.0, self.window_size.1 as f32);
        self.transform.proj = gen_ortho_mat(l, r, t, b, -1.0, 1.0);
        self.encoder.update_buffer(&self.data.transform, &[self.transform], 0).unwrap();
        self.encoder.clear(&self.color_view, BLACK);
        self.encoder.draw(&self.slice, &self.pso, &self.data);
        self.encoder.flush(device); // execute draw commands
    }
}
