//! A big list of possible items.
#![allow(dead_code)]

mod deser_structs;

use serde_yaml;
use comp::*;
use renderer::*;
use std::sync::RwLock;
use std::fs;
use std::collections::BTreeMap;

/// This is the type of equipment that an item is. It's either None, meaning
/// this item is not a piece of equipment, or one of the other values.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum EquipmentType {
    Helmet,
    Body,
    Weapon,
    Ring,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemType(usize);

#[derive(Serialize, Deserialize, Debug)]
struct AnimData {
    /// The animation we play in the associated animsprite resource
    anim_num: usize,
    num_frames: usize,
    frame_time: f32,
    /// See the comp::ANIM_SPRITE_* constants
    flags: u8,
}

/// Data to display an item in-world
#[derive(Serialize, Deserialize, Debug)]
struct InWorldGfx {
    /// The actual resource - either a texture key or animation (see anim_data
    /// value for which of these it is)
    tex_key: TextureKey,
    width: f32,
    height: f32,
    /// This is only Some if this is an animation
    anim_data: Option<AnimData>,
}

impl InWorldGfx {
    /// Create a drawable component (animsprite or staticsprite) to render this.
    fn as_drawable_component(&self) -> DrawableComponent {
        match self.anim_data {
            // Animation
            Some(ref data) => {
                DrawableComponent::Anim(AnimSprite::new(
                    self.width, self.height, data.frame_time,
                    data.num_frames, self.tex_key).with_flags(data.flags | ANIM_SPRITE_UPRIGHT))
            },
            // Static
            None => {
                DrawableComponent::Static(StaticSprite {
                    w: self.width,
                    h: self.height,
                    sprite: self.tex_key,
                    flags: STATIC_SPRITE_UPRIGHT
                })
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct EquipmentData {
    equipment_type: EquipmentType,
    /// The key of the animation to use to render this equipment on a creature.
    /// Animation should match with that creatures animations (animations and
    /// frames will be matched to give the illusion that the creature is
    /// 'wearing' the equipment).
    /// This is None if this is just a ring or weapon.
    anim_key: Option<TextureKey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemDetails {
    /// Graphics used to display this in-world (when on the floor)
    in_world_gfx: InWorldGfx,
    /// Icon in the inventory / other stores
    icon: TextureKey,
    /// If this is equipment, this is Some.
    equipment_data: Option<EquipmentData>,
    /// If true, this item can stack (up to 99) in the inventory. Otherwise,
    /// separate stacks will be maintained.
    stacks: bool,
    name: String,
}

impl ItemDetails {
    /// Returns either an animsprite or staticsprite which can be used to draw
    /// this in the world (rather than just an inventory icon)
    fn get_in_world_drawable(&self) -> DrawableComponent {
        self.in_world_gfx.as_drawable_component()
    }
}

struct ItemRegister {
    item_list: Vec<ItemDetails>,
}

impl ItemRegister {
    pub fn new() -> ItemRegister {
        ItemRegister {
            item_list: Vec::new(),
        }
    }

    fn get_details(&self, item: ItemType) -> &ItemDetails {
        debug_assert!(item.0 < self.item_list.len(), "Item ID out of range");
        &self.item_list[item.0 as usize]
    }
}

lazy_static! {
    /// The item register is a list of all the stats of all the items. Items
    /// should be referenced by their index in this list. This list will be
    /// loaded from a configuration file at game startup, and will be accessed
    /// read-only from then on.
    static ref ITEM_REGISTER : RwLock<ItemRegister> =
        RwLock::new(ItemRegister::new());
}

impl ItemType {
    /// Get the texture key for this item type
    pub fn get_icon_tex_key(self) -> TextureKey {
        ITEM_REGISTER.read().unwrap().get_details(self).icon
    }

    /// Returns either an animsprite or staticsprite which can be used to draw
    /// this in the world (rather than just an inventory icon)
    pub fn get_in_world_drawable(self) -> DrawableComponent {
        ITEM_REGISTER.read().unwrap().get_details(self).get_in_world_drawable()
    }


    pub fn equipment_type(self) -> Option<EquipmentType> {
        ITEM_REGISTER.read().unwrap().get_details(self)
            .equipment_data.map(|d| d.equipment_type)
    }

    /// Panics if this item is not equipment, or if this is some equipment that doesn't have an animation (i.e. rings)
    pub fn get_equipment_anim(self) -> TextureKey {
        ITEM_REGISTER.read().unwrap().get_details(self)
            .equipment_data.expect("Item is not equipment")
            .anim_key.expect("This equipment doesn't have an animation (is it a ring or weapon?)")
    }

    pub fn stacks(self) -> bool {
        ITEM_REGISTER.read().unwrap().get_details(self).stacks
    }
}

/// Given a string, get the item type with that name. This is a linear search
/// (i.e. pretty fucking slow), so this should really only be called sparingly
/// or at setup, like when setting up drop tables, or maybe an item search
/// function.
pub fn get_item_type_with_name(name: &str) -> Option<ItemType> {
    ITEM_REGISTER.read().unwrap().item_list.iter().enumerate()
        .find(|(_, i)| &i.name == name)
        .map(|(ix, _)| ItemType(ix))
}

/// Load the item definitions from the res files.
pub fn load_item_definitions() {
    // Hardcode items for now until we sort out YAML reading
    eprintln!("Warning: Not reading item defs from res files, just hardcoding items");

    // Load definitions
    let definitions = deser_structs::load_defs();

    // Get a list of item definitions, plus a map of item names to indices
    let mut item_name_map = BTreeMap::new();
    definitions.iter().enumerate().for_each(|(ix, (k, _))| { item_name_map.insert(k.clone(), ix); });
    let definitions : Vec<ItemDetails>
        = definitions.iter().map(|(k, v)| v.link_assets(k.clone())).collect();

    println!("Details: {:?}", definitions);

    let mut ir = ITEM_REGISTER.write().unwrap();
    definitions.into_iter().for_each(|d| ir.item_list.push(d));
}
