//! Components for the ECS

mod coll;

pub use self::coll::*;
use fpa::*;
use fpavec::*;
use specs::{DenseVecStorage, HashMapStorage};
use renderer::{TextureKey, self};

#[derive(Component)]
pub struct Pos {
    pub x: Fx32,
    pub y: Fx32,
}

impl Pos {
    pub fn to_vec(&self) -> Vec32 { Vec32::new(self.x, self.y) }
}

#[derive(Component)]
pub struct Vel {
    pub x: Fx16,
    pub y: Fx16,
}

impl Vel {
    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vec16 { Vec16::new(self.x, self.y) }
}

/// Any entity with this component will be controlled as if it was a player
/// entity.
#[derive(Component)]
pub struct PlayerControlled {
    pub move_speed: Fx16,
}

/// Draw a static sprite, using the Pos component as the bottom centre
#[derive(Component)]
pub struct StaticSprite {
    pub w: f32,
    pub h: f32,
    pub sprite: TextureKey,
}

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
}

impl AnimSprite {
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
