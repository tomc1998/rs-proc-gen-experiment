//! Components for the ECS

mod coll;
mod control;
mod health;
mod visual;

pub use self::coll::*;
pub use self::control::*;
pub use self::health::*;
pub use self::visual::*;
use fpa::*;
use fpavec::*;
use specs::{DenseVecStorage, VecStorage};

#[derive(Clone, Component)]
pub struct Pos {
    pub pos: Vec32,
}

#[derive(Component)]
pub struct Vel {
    pub vel: Vec16,
}

/// When this component is attached to an object, it will remove the object once
/// the given time has elapsed.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Lifetime {
    /// Lifetime of this in millis. Will count down to 0.
    pub lifetime: Fx32,
}
