use specs::*;
use vec::*;

#[allow(dead_code)]
pub const SLIME_MOVE_SPEED : f32 = 100.0;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlayerState {
    Default,
    Attacking,
}

/// Any entity with this component will be controlled as if it was a player
/// entity.
#[derive(Component)]
pub struct PlayerControlled {
    pub move_speed: f32,
    pub state: PlayerState,
    /// Attack time in millis, set to some val which then counts down to 0 when
    /// state is Attacking,, then state is set to Default.
    pub attack_time: f32,
}

impl PlayerControlled {
    pub fn new() -> PlayerControlled {
        PlayerControlled {
            move_speed: 100.0,
            state: PlayerState::Default,
            attack_time: 0.0,
        }
    }
}

#[allow(dead_code)]
pub enum SlimeState {
    Idle,
    Charging,
    Jumping
}

/// Any entity with this component will be controlled as if it was a player
/// entity.
#[derive(Component)]
pub struct AISlime {
    /// Set when moving towards a place in idle
    pub move_target: Vec32,
    /// Set when attacking
    pub attack_target: Option<Entity>,
    /// Once this hits 0, the slime changes state. This is used when charging,
    /// but also when jumping to know when to reset to idle.
    pub charge_time: f32,
    pub state: SlimeState,
}
