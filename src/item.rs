//! A big list of possible items.

use comp::*;
use renderer::TextureKey;
use equipment::*;

/// This is the type of equipment that an item is. It's either None, meaning
/// this item is not a piece of equipment, or one of the other values.
pub enum EquipmentType {
    None,
    Helmet,
    Body,
    Weapon,
    Ring,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ItemType {
    /// Store money as an item, like in Runescape. This means we can have
    /// different types for different currencies, which is really cool!
    Money,
    BronzeHelmet,
}

impl ItemType {
    /// Get the texture key for this item type
    pub fn get_icon_tex_key(&self) -> TextureKey {
        match self {
            ItemType::Money => TextureKey::IconMoney,
            ItemType::BronzeHelmet => TextureKey::IconBronzeHelmet
        }
    }

    /// Returns either an animsprite or staticsprite which can be used to draw
    /// this in the world (rather than just an inventory icon)
    pub fn get_in_world_drawable(&self) -> Option<DrawableComponent> {
        match self {
            ItemType::Money => Some(DrawableComponent::Anim(
                AnimSprite::new(16.0, 16.0, 40.0, 6, TextureKey::GoldCoinAnim))),
            ItemType::BronzeHelmet => Some(
                DrawableComponent::Static(StaticSprite {
                    w: 16.0, h: 16.0,
                    sprite: TextureKey::IconBronzeHelmet
                }))
        }
    }

    pub fn equipment_type(&self) -> EquipmentType {
        match self {
            ItemType::BronzeHelmet => EquipmentType::Helmet,
            _ => EquipmentType::None,
        }
    }

    /// Convert this to a Helmet value (see equipment.rs). Panics if this isn't
    /// a helmet.
    pub fn as_helmet(&self) -> Helmet {
        match self {
            ItemType::BronzeHelmet => Helmet::BronzeHelmet,
            _ => panic!("Not a helmet")
        }
    }

    /// Convert this to a Helmet value (see equipment.rs). Panics if this isn't
    /// a helmet.
    pub fn as_body(&self) -> Body {
        match self {
            _ => panic!("Not a body")
        }
    }

    /// Convert this to a Helmet value (see equipment.rs). Panics if this isn't
    /// a helmet.
    pub fn as_weapon(&self) -> Weapon {
        match self {
            _ => panic!("Not a weapon")
        }
    }

    /// Convert this to a Helmet value (see equipment.rs). Panics if this isn't
    /// a helmet.
    pub fn as_ring(&self) -> Ring {
        match self {
            _ => panic!("Not a ring")
        }
    }
}
