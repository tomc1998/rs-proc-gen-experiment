use specs::*;

/// Apply a tint to this entity when drawing. This will apply to StaticSprite
/// and AnimSprite components.
#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Tint {
    pub col: [f32; 4],
}

/// Visually rotate whatever we're rendering
#[derive(Component)]
pub struct Rot {
    /// Rotation in radians, anti-clockwise
    pub angle: f32,
}
