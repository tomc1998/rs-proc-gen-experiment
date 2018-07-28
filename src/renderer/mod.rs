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

    /// Pipeline data
    data: pipe::Data<Resources>,

    pso: gfx::pso::PipelineState<Resources, pipe::Meta>,

    /// Transform uniform block, to avoid repeated allocations on render() calls
    transform: Transform,
}

impl Renderer {
    // fn load_assets(factory: &mut Factory) -> (TextureAtlas<TextureKey>, ShaderResourceView<Resources, [f32; 4]>) {
    //     let mut builder = AtlasBuilder::<TextureKey>::new(512, 512);

    //     // Create frame sets
    //     let human_frame_set = builder.add_frame_set(FrameSet {
    //         frames: vec![
    //             Frames::Ordered(0,  3),  // Walk down
    //             Frames::Ordered(4,  7),  // Walk up
    //             Frames::Ordered(8,  11), // Walk right
    //             Frames::Ordered(12, 15), // Walk left
    //             Frames::Ordered(0,  0),  // Idle down
    //             Frames::Ordered(4,  4),  // Idle up
    //             Frames::Ordered(8,  8),  // Idle right
    //             Frames::Ordered(12, 12), // Idle left
    //             Frames::Ordered(1,  1),  // Attack down
    //             Frames::Ordered(5,  5),  // Attack up
    //             Frames::Ordered(9,  9),  // Attack right
    //             Frames::Ordered(13, 13), // Attack left
    //             ]
    //     });

    //     let gold_coin_frame_set = builder.add_frame_set(FrameSet {
    //         frames: vec![Frames::Ordered(0,  5)]
    //     });
    //     let slice_frame_set = builder.add_frame_set(FrameSet {
    //         frames: vec![Frames::Ordered(0,  4)]
    //     });
    //     let slime_frame_set = builder.add_frame_set(FrameSet {
    //         frames: vec![Frames::Ordered(0,  0)]
    //     });

    //     // Now build all the rest of the atlas
    //     builder
    //         .set_font("res/open-sans.ttf",
    //                   Charset::alpha()
    //                   .and(Charset::number())
    //                   .and(Charset::common_punc())
    //                   .into_iter(),
    //                   32.0).unwrap()
    //         .add_tex("White", "res/white.png").unwrap()
    //         .add_tex("GreenTree00", "res/sprites/green-tree-00.png").unwrap()
    //         .add_tex("InventoryMockup", "res/sprites/ui/inventory-mockup.png").unwrap()
    //         .add_tex("IconMoney", "res/sprites/icon/money.png").unwrap()
    //         .add_tileset("TilesetGrass", "res/tileset-grass.png", 8, 8).unwrap()
    //         .add_anim_sprite("res/sprites/human-00.png", "Human00Anim",
    //                          human_frame_set, 8, 8).unwrap()
    //         .add_anim_sprite("res/sprites/equipment/bronze/helmet.png", "BronzeHelmetAnim",
    //                          human_frame_set, 8, 8).unwrap()
    //         .add_anim_icon("IconBronzeHelmet", "BronzeHelmetAnim",
    //                        0.0, -0.125, 1.0, 0.75)
    //         .add_anim_sprite("res/sprites/pickup/gold-coin.png", "GoldCoinAnim",
    //                          gold_coin_frame_set, 8, 8).unwrap()
    //         .add_anim_sprite("res/sprites/slime-00.png", "SlimeAnim",
    //                          slime_frame_set, 8, 8).unwrap()
    //         .add_anim_sprite("res/sprites/fx/slice-00.png", "SliceAnim",
    //                          slice_frame_set, 16, 16).unwrap()
    //         .add_bitmap_font("FontTinyNumbers", "res/sprites/ui/tiny-numbers.png",
    //                          &[('1', (0, 0)), ('2', (1, 0)), ('3', (2, 0)), ('4', (3, 0)), ('5', (4, 0)),
    //                            ('6', (0, 1)), ('7', (1, 1)), ('8', (2, 1)), ('9', (3, 1)), ('0', (4, 1))][..],
    //                          3, 5).unwrap()
    //         .build(factory)
    // }

    pub fn new(factory: &mut Factory,
               color_view: RenderTargetView<Resources, ColorFormat>,
               depth_view: gfx::handle::DepthStencilView<Resources, DepthFormat>,
               settings: RendererSettings) -> (Renderer, TextureAtlas<TextureKey>) {
        use gfx::{Factory, traits::FactoryExt};

        // Load the texture atlas
        // let (atlas, tex_view) = Renderer::load_assets(factory);
        let (atlas, tex_view) = asset_loader::load_assets(factory);
        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(
                gfx::texture::FilterMethod::Scale,
                gfx::texture::WrapMode::Clamp));

        // Create the encoder
        let encoder : gfx::Encoder<_, _> = factory.create_command_buffer().into();

        // Allocate buffers
        let transform_buffer = factory.create_constant_buffer(1);

        let vertex_buffer = factory.create_buffer::<Vertex>(
            V_BUF_SIZE, Role::Vertex, Usage::Dynamic, Bind::SHADER_RESOURCE).unwrap();

        let transform = Transform {
            proj: [[0.0; 4]; 4], // Filled in by camera later
            view: [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [0.0, 0.0, 0.0, 1.0]],
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
            settings: settings,
            encoder: encoder,
            data: data,
            pso: pso,
            transform: transform,
        }, atlas)
    }

    fn rect(v_buf: &mut [Vertex], tex: &UvRect,
            x: f32, y: f32, z: f32,
            w: f32, h: f32,
            col: [f32; 4]) {
        debug_assert!(v_buf.len() >= 6, "Drawing rect but v_buf < 6 in len");
        v_buf[0] = Vertex {pos: [x, y, z], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [x+w, y, z], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [x+w, y+h, z], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [x, y, z], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [x, y+h, z], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [x+w, y+h, z], col: col, uv: [tex.right, tex.bottom]};
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
        v_buf[0] = Vertex {pos: [p0[0], p0[1], z], col: col, uv: [tex.left, tex.top]};
        v_buf[1] = Vertex {pos: [p1[0], p1[1], z], col: col, uv: [tex.right, tex.top]};
        v_buf[2] = Vertex {pos: [p2[0], p2[1], z], col: col, uv: [tex.right, tex.bottom]};
        v_buf[3] = Vertex {pos: [p0[0], p0[1], z], col: col, uv: [tex.left, tex.top]};
        v_buf[4] = Vertex {pos: [p3[0], p3[1], z], col: col, uv: [tex.left, tex.bottom]};
        v_buf[5] = Vertex {pos: [p2[0], p2[1], z], col: col, uv: [tex.right, tex.bottom]};
    }

    pub fn update_window_size(&mut self, window: &GlWindow) {
        // Update the render target size
        gfx_window_glutin::update_views(window, &mut self.data.out_col, &mut self.data.out_depth);
    }

    /// Clear the screen.
    pub fn clear(&mut self) {
        self.encoder.clear(&self.data.out_col, BLACK);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);
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
        self.encoder.draw(&slice, &self.pso, &self.data);
        self.encoder.flush(device); // execute draw commands
    }
}
