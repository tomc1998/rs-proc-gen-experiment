use specs::{DenseVecStorage};
use fpavec::*;
use fpa::*;

pub const SLIME_MOVE_SPEED : Fx16 = Fx16((100.0 * FPA_MUL) as i16);

/// Any entity with this component will be controlled as if it was a player
/// entity.
#[derive(Component)]
pub struct PlayerControlled {
    pub move_speed: Fx16,
}

pub enum SlimeState {
    Idle,
    Targeted,
    Charging,
    Jumping
}

/// Any entity with this component will be controlled as if it was a player
/// entity.
#[derive(Component)]
pub struct AISlime {
    pub move_target: Vec32,
    /// Once this hits 0, the slime jumps
    pub charge_time: Fx16,
    pub state: SlimeState,
}
