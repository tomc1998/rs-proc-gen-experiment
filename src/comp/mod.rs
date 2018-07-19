//! Components for the ECS

mod coll;
mod control;
mod health;

pub use self::coll::*;
pub use self::control::*;
pub use self::health::*;
use fpa::*;
use fpavec::*;
use specs::{DenseVecStorage, HashMapStorage, VecStorage};
use renderer::{TextureKey, self};

#[derive(Clone, Component)]
pub struct Pos {
    pub pos: Vec32,
}

#[derive(Component)]
pub struct Vel {
    pub vel: Vec16,
}

/// Draw a static sprite, using the Pos component as the bottom centre
#[derive(Component)]
pub struct StaticSprite {
    pub w: f32,
    pub h: f32,
    pub sprite: TextureKey,
}

/// If set, the anim sprite will not loop.
pub const ANIM_SPRITE_NO_LOOP : u8 = 1;

/// Draw an animated sprite, using the Pos component as the bottom centre
#[derive(Component)]
pub struct AnimSprite {
    pub w: f32,
    pub h: f32,
    pub curr_frame: usize,
    /// Frame time in millis
    pub frame_time: Fx32,
    /// Frame time counter
    pub curr_frame_time: Fx32,
    /// When curr_frame == num_frames, curr_frame will be set to 0.
    pub num_frames: usize,
    /// The key of the animation
    pub anim: TextureKey,
    /// See the ANIM_SPRITE_* constants
    pub flags: u8,
}

impl AnimSprite {
    pub fn new(w: f32, h: f32, frame_time: Fx32, num_frames: usize, anim: TextureKey) -> AnimSprite {
        AnimSprite {
            w: w,
            h: h,
            curr_frame: 0,
            frame_time: frame_time,
            curr_frame_time: Fx32::new(0.0),
            num_frames: num_frames,
            anim: anim,
            flags: 0,
        }
    }

    pub fn with_flags(mut self, flags: u8) -> AnimSprite {
        self.flags = flags;
        self
    }

    /// Change the current anim, resetting all counters
    pub fn set_anim(&mut self, anim: TextureKey, num_frames: usize, frame_time: Fx32) {
        if self.anim != anim {
            self.curr_frame_time = Fx32::new(0.0);
            self.curr_frame = 0;
        }
        self.anim = anim.clone();
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
            TilesetEnum::Grass => renderer::TextureKey::TilesetGrass,
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

/// When this component is attached to an object, it will remove the object once
/// the given time has elapsed.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Lifetime {
    /// Lifetime of this in millis. Will count down to 0.
    pub lifetime: Fx32,
}
