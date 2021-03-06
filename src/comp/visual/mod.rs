use specs::{DenseVecStorage, HashMapStorage};
use renderer::{TextureKey, self, get_asset_by_name};

mod fx;
mod camera;

pub use self::fx::*;
pub use self::camera::*;

/// An enum which represents either an animated sprite or static sprite
pub enum DrawableComponent {
    #[allow(dead_code)]
    Static(StaticSprite),
    Anim(AnimSprite),
}

/// Draw a static sprite, using the Pos component as the bottom centre
#[derive(Component)]
pub struct StaticSprite {
    pub w: f32,
    pub h: f32,
    pub sprite: TextureKey,
    pub flags: u8,
}

/// If set, the anim sprite will not loop.
pub const ANIM_SPRITE_NO_LOOP : u8 = 1;
/// If set, draw this 'upright' rather than on the horizontal plane
pub const ANIM_SPRITE_UPRIGHT : u8 = 2;

/// If set, draw this 'upright' rather than on the horizontal plane
pub const STATIC_SPRITE_UPRIGHT : u8 = 1;

/// Draw an animated sprite, using the Pos component as the bottom centre
#[derive(Component)]
pub struct AnimSprite {
    pub w: f32,
    pub h: f32,
    pub curr_frame: usize,
    /// Frame time in millis
    pub frame_time: f32,
    /// Frame time counter
    pub curr_frame_time: f32,
    /// When curr_frame == num_frames, curr_frame will be set to 0.
    pub num_frames: usize,
    /// The key of the animation
    pub anim_key: TextureKey,
    /// The number of the animation we're playing from the animsprite resource.
    pub anim: usize,
    /// See the ANIM_SPRITE_* constants
    pub flags: u8,
}

impl AnimSprite {
    pub fn new(w: f32, h: f32, frame_time: f32, num_frames: usize, anim: TextureKey) -> AnimSprite {
        AnimSprite {
            w: w,
            h: h,
            curr_frame: 0,
            frame_time: frame_time,
            curr_frame_time: 0.0,
            num_frames: num_frames,
            anim_key: anim,
            anim: 0,
            flags: 0,
        }
    }

    pub fn with_flags(mut self, flags: u8) -> AnimSprite {
        self.flags = flags;
        self
    }

    /// Change the current anim, resetting all counters
    pub fn set_anim(&mut self, anim: usize, num_frames: usize, frame_time: f32) {
        if self.anim != anim {
            self.curr_frame_time = 0.0;
            self.curr_frame = 0;
        }
        self.anim = anim;
        self.num_frames = num_frames;
        self.frame_time = frame_time;
    }
}

/// An anum of tilesets. Named this way to avoid collisions with
/// renderer::atlas::Tileset.
#[derive(Clone, Copy, Debug)]
pub enum TilesetEnum {
    /// 0 - Dirt
    /// 1 - Grass
    /// 2 - Water
    Grass,
}

impl TilesetEnum {
    pub fn convert_to_tex_key(&self) -> renderer::TextureKey {
        match *self {
            TilesetEnum::Grass => get_asset_by_name("TilesetGrass"),
        }
    }
}

/// The width / height of tilemaps.
pub const TILEMAP_SIZE : usize = 16;

/// A tilemap component. Coupled with a Pos component (for tilemap-wise offset,
/// see below), this will render a tilemap at a given position with the given
/// tileset.
/// Width is defined by TILEMAP_SIZE.
/// The pos is multiplied by the chunk size, so a tilemap with pos (1, 0) and
/// one with (2, 0) will be adjacent to one another.
#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Tilemap {
    pub tileset: TilesetEnum,
    /// Each u8 will correspond to a tile. This is defined implicitly by the
    /// TilesetEnum.
    pub data: [u8; TILEMAP_SIZE * TILEMAP_SIZE],
}

