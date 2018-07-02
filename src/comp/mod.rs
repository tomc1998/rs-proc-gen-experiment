//! Components for the ECS

use specs::{DenseVecStorage};

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
pub struct DebugRender {
    pub col: [f32; 4],
    pub w: f32,
    pub h: f32,
}
