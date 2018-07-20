use specs::*;

/// Marker for an entity. The camera will follow the first entity with a FollowCamera component.
#[derive(Clone, Copy, Default, Component)]
#[storage(NullStorage)]
pub struct FollowCamera;
