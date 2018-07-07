//! Components for the ECS

use specs::{DenseVecStorage, HashMapStorage};
use renderer::{TextureKey, self};

#[derive(Component)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Vel {
    pub x: f32,
    pub y: f32,
}

/// Any entity with this component will be controlled as if it was a player
/// entity.
#[derive(Component)]
pub struct PlayerControlled {
    pub move_speed: f32,
}

/// Temporary component for quick rendering system testing. Will probably be
/// removed, but tells the renderer to draw a coloured rect at the Pos of this
/// entity. Centred.
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
    pub anim: TextureKey,
}

impl AnimSprite {
    /// Change the current anim, resetting all counters
    pub fn set_anim(&mut self, anim: TextureKey, num_frames: usize, frame_time: f32) {
        if self.anim != anim {
            self.curr_frame_time = 0.0;
            self.curr_frame = 0;
        }
        self.anim = anim.clone();
        self.num_frames = num_frames;
        self.frame_time = frame_time;
    }
}

/// Temporary component for quick rendering system testing. Will probably be
/// removed, but tells the renderer to draw a coloured rect at the Pos of this
/// entity. Centred.
#[derive(Component)]
pub struct DebugRender {
    pub col: [f32; 4],
    pub w: f32,
    pub h: f32,
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

/// A tilemap component. Coupled with a Pos component (for offset), this will
/// render a tilemap at a given position with the given tileset.
/// Width is defined by TILEMAP_SIZE.
#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Tilemap {
    pub tileset: TilesetEnum,
    /// Each u8 will correspond to a tile. This is defined implicitly by the
    /// TilesetEnum.
    pub data: [u8; TILEMAP_SIZE * TILEMAP_SIZE],
}
