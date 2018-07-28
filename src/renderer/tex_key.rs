use std::sync::RwLock;
use std::collections::BTreeMap;

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

pub type TextureKey = usize;


lazy_static! {
    /// This shouldn't really be accessed directly, except for in the
    /// asset_loader module, where it is first initially written to.
    pub static ref ASSET_NAME_MAP : RwLock<BTreeMap<String, TextureKey>>
        = RwLock::new(BTreeMap::new());
}

/// Panics if asset not found
pub fn get_asset_by_name(name: &str) -> TextureKey {
    *ASSET_NAME_MAP.read().unwrap().get(name)
        .expect(&format!("Asset not found: {}", name))
}
