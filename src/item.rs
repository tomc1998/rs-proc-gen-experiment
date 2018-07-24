//! A big list of possible items.

use comp::*;
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
    pub fn get_icon_tex_key(&self) -> TextureKey {
        match self {
            ItemType::Money => TextureKey::IconMoney
        }
    }

    /// Returns either an animsprite or staticsprite which can be used to draw
    /// this in the world (rather than just an inventory icon)
    pub fn get_in_world_drawable(&self) -> Option<DrawableComponent> {
        match self {
            ItemType::Money => Some(DrawableComponent::Anim(
                AnimSprite::new(16.0, 16.0, 40.0, 6, TextureKey::GoldCoinAnim))),
        }
    }
}
