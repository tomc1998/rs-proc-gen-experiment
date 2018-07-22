//! Module for components that control on-death events

use drop_tables::*;
use specs::*;

/// For entities that drop something on death (i.e. on entity deletion,
/// regardless of how it happened, for example if this is added to a projectile
/// this will trigger once the projectile hits something and is removed)
/// NOTE: Entities must have an associated Pos component to actually drop
/// anything - this would be nonsensical otherwise, as there would be nowhere
/// for the items to drop!
#[derive(Component)]
pub struct OnDeathDrop {
    /// A reference to the drop table to use
    pub drop_table: DropTableKey,
    /// How many drops to process (i.e. how many separate queries to the drop
    /// table to do, and how many stacks will be dropped. This is the minimum
    /// value for this (Inclusive))
    pub min_drops: u8,
    /// Maxmimum number of drops to process (Exclusive)
    pub max_drops: u8,
}
