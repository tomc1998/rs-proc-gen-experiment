use specs::{DenseVecStorage};
use fpa::*;

/// A hitmask - only hurtboxes that have an overlap with a health hitmask will
/// hit (i.e. if we have the hurtbox hitmask as hb_hm, and the health hitmask as
/// he_hm, health will only be deducted upon collision if hb_hm & he_hm > 0)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Hitmask(pub u8);

pub const HITMASK_PLAYER       : u8 = 0b00000001;
pub const HITMASK_ALLY         : u8 = 0b00000010;
pub const HITMASK_ENEMY        : u8 = 0b00000100;
pub const HITMASK_NEUTRAL      : u8 = 0b00001000;
pub const HITMASK_DESTRUCTIBLE : u8 = 0b00010000;

#[allow(dead_code)]
impl Hitmask {
    /// A default hitmask for enemy attacks. Hits players, allies, and neutrals.
    pub fn default_enemy_attack() -> Hitmask {
        Hitmask(HITMASK_PLAYER | HITMASK_ALLY | HITMASK_NEUTRAL)
    }

    /// A default hitmask for player attacks. Hits neutrals, destructibles, and enemies
    pub fn default_player_attack() -> Hitmask {
        Hitmask(HITMASK_ENEMY | HITMASK_DESTRUCTIBLE | HITMASK_NEUTRAL)
    }

    /// A default hitmask for player attacks. Hits enemies only.
    pub fn default_ally_attack() -> Hitmask {
        Hitmask(HITMASK_ENEMY)
    }

    /// A default hitmask for player attacks. Hits players, enemies, allies, and other neutrals.
    pub fn default_neutral_attack() -> Hitmask {
        Hitmask(HITMASK_PLAYER | HITMASK_ALLY | HITMASK_NEUTRAL | HITMASK_ENEMY)
    }

    /// Custom hitmask num
    pub fn with_num(num: u8) -> Hitmask {
        Hitmask(num)
    }

    /// Set this hitmask to a value
    pub fn set(&mut self, num: u8) {
        self.0 = num
    }

    /// Check if 2 hitmasks collide
    pub fn collides(&self, other: &Hitmask) -> bool {
        self.0 & other.0 > 0
    }
}

/// Comp that indicates this entity has health, which can be removed via
/// combat.. Once health is reduced to 0, the entity will be removed. This also
/// contains the entity's stats, such as resistances.
#[derive(Component)]
pub struct Health {
    pub max_health: u8,
    pub health: u8,
    /// Maximum invuln time after being hit (in millis)
    pub max_inv_time: Fx16,
    /// Counts to 0
    pub inv_time: Fx16,
    /// What is this?
    /// If this is the component for an ally, for example, this should have a
    /// value of HITMASK_ALLY.
    /// This will be hit by hurtboxes which have HITMASK_ALLY as part of their
    /// hitmask.
    pub mask: Hitmask,
}

impl Health {
    pub fn new(max_health: u8, mask: Hitmask) -> Health {
        Health {
            max_health: max_health,
            health: max_health,
            mask: mask,
            max_inv_time: Fx16::new(300.0),
            inv_time: Fx16::new(0.0),
        }
    }

    /// Hurt this health component with the hurt component. Returns true if this
    /// entity should die now.
    pub fn hurt(&mut self, hurt: &Hurt) -> bool {
        // Check that we actually collide with this thing
        if !self.mask.collides(&hurt.mask) { return false; }

        if self.health > hurt.damage {
            self.health -= hurt.damage;
            false
        } else {
            self.health = 0;
            true
        }
    }
}

/// If this is set, the hurt component will be removed once it hurts one thing.
/// This is useful for projectile attacks.
#[allow(dead_code)]
pub const HURT_DIES : u8 = 1;

/// If an entity contains this, this means that if it collides with another
/// entity that has health, it will reduce the health of that entity.
#[derive(Component)]
pub struct Hurt {
    /// How much damage to inflict (before resistances)
    pub damage: u8,
    /// What does this hurt?
    pub mask: Hitmask,
    /// Some flags. See the HURT_* consts.
    pub flags: u8,
}
