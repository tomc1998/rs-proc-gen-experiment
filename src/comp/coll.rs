use vec::*;
use specs::{DenseVecStorage};

pub trait Coll<C> {
    /// Try and resolve this collision by shifting us some vector. Assume the
    /// other object is static and will not resolve a collision.
    /// This won't be called if the flags has COLL_STATIC.
    fn resolve(&self, other: &C, self_pos: Vec32, other_pos: Vec32) -> Vec32;

    /// Get the AABB bounding box for this collision shape (i.e. top left, bottom right point)
    fn aabb(&self, pos: Vec32) -> [Vec32; 2];

    /// Return this collision object's flags
    fn flags(&self) -> u8;
}

/// Does this collision body affect the physics of entities?
pub const COLL_SOLID : u8 = 1;
/// Is this body moved by other solid bodies? (assumes COLL_SOLID = 1)
pub const COLL_STATIC : u8 = 2;

#[derive(Component)]
pub struct CollCircle {
    pub r: f32,
    pub off: Vec32,
    pub flags: u8,
}

impl Coll<CollCircle> for CollCircle {
    fn resolve(&self, other: &CollCircle, self_pos: Vec32, other_pos: Vec32) -> Vec32 {
        let dis = ((other_pos + other.off) - (self_pos + self.off)).len();
        let combined_r = self.r + other.r;
        if dis < combined_r {
            let vec = (self_pos + self.off) - (other_pos + other.off);
            if vec.x == 0.0 && vec.y == 0.0 {
                Vec32::new(combined_r, 0.0)
            } else {
                (vec.nor() * (combined_r - dis))
            }
        } else {
            Vec32::new(0.0, 0.0)
        }
    }

    fn aabb(&self, pos: Vec32) -> [Vec32; 2] {
        [Vec32::new(pos.x - self.off.x, pos.y - self.off.y),
         Vec32::new(pos.x + self.off.x, pos.y + self.off.y)]
    }

    fn flags(&self) -> u8 { self.flags }
}
