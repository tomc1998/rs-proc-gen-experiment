//! Components for the ECS

mod coll;
mod control;
mod health;
mod visual;
mod alliance;
mod death;

pub use self::coll::*;
pub use self::control::*;
pub use self::health::*;
pub use self::visual::*;
pub use self::alliance::*;
pub use self::death::*;
use inventory::InventoryItem;
use vec::*;
use specs::*;
use equipment::*;

/// Track the position of a given entity, as long as that entity is still alive.
/// This assumes both this entity and the other entity have a position
/// component. Rendered offsets are from the bottom centre.
#[derive(Clone, Component)]
pub struct TrackPos {
    pub e: Entity,
    /// The offset of this entity.
    pub offset: Vec32,
}

/// Similar to trackpos, but simply copies over the anim number and frame
/// number.
/// This assumes the two animations have the same framesets. If they don't, this
/// could lead to a panic.
#[derive(Clone, Component)]
pub struct MatchAnim {
    pub e: Entity,
}

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
    pub lifetime: f32,
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
    pub magnet_radius: f32,
}

/// Equipment which should be drawn on a character and affect their stats in
/// battle
#[derive(Component, Default)]
pub struct Equipment {
    pub helmet: Option<Helmet>,
    pub body: Option<Body>,
    pub weapon: Option<Weapon>,
    pub ring: Option<Ring>,
}
