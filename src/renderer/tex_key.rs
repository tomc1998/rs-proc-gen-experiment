use std::sync::RwLock;
use std::collections::BTreeMap;

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
