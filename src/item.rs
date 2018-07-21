//! A big list of possible items.

use renderer::TextureKey;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ItemType {
    /// Store money as an item, like in Runescape. This means we can have
    /// different types for different currencies, which is really cool!
    Money,
}

impl ItemType {
    /// Get the texture key for this item type
    pub fn get_tex_key(&self) -> TextureKey {
        match self {
            ItemType::Money => TextureKey::IconMoney
        }
    }
}
