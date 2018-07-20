/// Textures available in the atlas
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TextureKey {
    // White texture for drawing just colour
    White,

    // Tilesets
    TilesetGrass,

    // Sprites
    Human00IdleDown,
    Human00IdleUp,
    Human00IdleLeft,
    Human00IdleRight,
    Human00WalkDown,
    Human00WalkUp,
    Human00WalkLeft,
    Human00WalkRight,
    Human00AttackDown,
    Human00AttackUp,
    Human00AttackLeft,
    Human00AttackRight,
    GreenTree00,
    Slime00Idle,
    Slime00Charge,
    Slice00,

    // UI
    InventoryMockup,
}
