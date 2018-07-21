//! Components for the ECS

mod coll;
mod control;
mod health;
mod visual;
mod alliance;

pub use self::coll::*;
pub use self::control::*;
pub use self::health::*;
pub use self::visual::*;
pub use self::alliance::*;
use inventory::InventoryItem;
use fpa::*;
use fpavec::*;
use specs::{DenseVecStorage, VecStorage};

#[derive(Clone, Component)]
pub struct Pos {
    pub pos: Vec32,
}

#[derive(Component)]
pub struct Vel {
    pub vel: Vec32,
}

/// When this component is attached to an object, it will remove the object once
/// the given time has elapsed.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Lifetime {
    /// Lifetime of this in millis. Will count down to 0.
    pub lifetime: Fx32,
}

/// Something that can be picked up and placed in an inventory / bag
#[derive(Component)]
pub struct Pickup {
    /// The item that this will add to the inventory if it's picked up
    pub item: InventoryItem,
}

/// Something that can collect items (items with a Pickup component). Currently
/// items will just be picked up into the player's inventory - other inventories
/// may be implemented later though.
#[derive(Component)]
pub struct Collector {
    /// The range of the magnetism that attracts pickups
    pub magnet_radius: Fx16,
}
