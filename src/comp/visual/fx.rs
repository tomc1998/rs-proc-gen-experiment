use specs::*;

/// Apply a tint to this entity when drawing. This will apply to StaticSprite
/// and AnimSprite components.
#[derive(Component)]
pub struct Tint {
    pub col: [f32; 4],
}
