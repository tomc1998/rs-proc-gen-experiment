/// Textures available in the atlas
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
// pub enum TextureKey {
//     // White texture for drawing just colour
//     White,

//     // Tilesets
//     TilesetGrass,

//     // Sprites
//     Human00Anim,
//     BronzeHelmetAnim,
//     GoldCoinAnim,
//     SlimeAnim,
//     SliceAnim,
//     GreenTree00,

//     // UI
//     InventoryMockup,

//     // Icons (for stuff like inventory)
//     IconMoney,
//     IconBronzeHelmet,

//     // Fonts
//     FontTinyNumbers
// }

pub type TextureKey = &'static str;
