//! A module for maintaining drop tables

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use fpa::*;
use item::*;

/// A key for accessing drop tables
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum DropTableKey {
    // Drop table for slimes
    Slime
}

/// A possible drop
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Drop {
    pub item: ItemType,
    /// The minimum number of items in this stack (Inclusive)
    pub min_num: u8,
    /// The maximum number of items in this stack (Exclusive)
    pub max_num: u8,
}

/// A drop table. Contains a list of items, along with relative probability
/// chances that they drop.
/// # NOTE: Probabilities are from 0 to 10000, to give us more fidelity with
/// fixed-point arithmetic
#[derive(PartialEq, Eq)]
pub struct DropTable {
    /// Items are assumed to be ordered by their probability (i.e. the first
    /// item in the tuple, the probability, should only ascend up to the maximum
    /// value of 10000.0.
    /// If the first item is listed at X probability, anything less than X is
    /// considered no drop.
    /// If the last itme is listed at X probability, anything equal to or above
    /// X is considered this drop.
    pub items: Vec<(Fx32, Drop)>,
}

impl DropTable {
    /// Given a number between 0 and 100,
    pub fn get_drop(&self, probability: Fx32) -> Option<Drop> {
        let mut curr_drop = None;
        for (p, d) in &self.items {
            if (probability - *p).0 < 0 {
                break;
            } else {
                curr_drop = Some(*d);
            }
        }
        curr_drop
    }
}

/// Maps droptablekeys to droptables
#[derive(PartialEq, Eq)]
pub struct DropTableMap(BTreeMap<DropTableKey, DropTable>);

impl Deref for DropTableMap {
    type Target = BTreeMap<DropTableKey, DropTable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for DropTableMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DropTableMap {
    /// Create a new standard drop table map which has the normal drops and
    /// probabilities
    pub fn new_standard_map() -> DropTableMap {
        let mut map = BTreeMap::new();
        map.insert(DropTableKey::Slime,
                   DropTable {items: vec![
                       (Fx32::new(2500.0), Drop { item: ItemType::Money, min_num: 1, max_num: 3 }),
                       (Fx32::new(7500.0), Drop { item: ItemType::Money, min_num: 3, max_num: 9 }),
                       (Fx32::new(9000.0), Drop { item: ItemType::Money, min_num: 9, max_num: 20 }),
                   ]});
        DropTableMap(map)
    }
}
