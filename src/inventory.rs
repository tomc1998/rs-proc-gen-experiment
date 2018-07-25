use item::{ItemType, EquipmentType};
use ui::InventorySlotRef;

pub const INVENTORY_SIZE: usize = 18;

/// An item in the inventory.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InventoryItem {
    pub item_type: ItemType,
    /// Each inventory slot can hold up to 99 slots (any more and it gets hard
    /// to render in the inventory screen!)
    /// If this is 0, this is considered a bad state. If an item slot is at 0,
    /// it should be set to None in the Inventory struct.
    pub num: u8,
}

impl InventoryItem {
    pub fn new(item_type: ItemType, num: u8) -> InventoryItem {
        debug_assert!(num > 0, "Creating an inventory item with 0 as the num");
        debug_assert!(num < 100, "Creating an inventory item with more than 99 items");
        InventoryItem { item_type, num }
    }
}

/// The player's inventory
/// This needs to be an ordered data structure, because the order of items in
/// the inventory is important to the player.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inventory {
    /// The items in this inventory. Non indicates the slot is empty.
    pub items: Box<[Option<InventoryItem>; INVENTORY_SIZE]>,
    pub helmet: Option<InventoryItem>,
    pub body: Option<InventoryItem>,
    pub weapon: Option<InventoryItem>,
    pub ring: Option<InventoryItem>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Box::new([None; INVENTORY_SIZE]),
            .. Default::default()
        }
    }

    /// Add an item to the inventory, stacking if possible, returning false if
    /// the inventory is full.
    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        // First check for places to stack
        for i_slot in self.items.iter_mut() {
            match i_slot {
                Some(i) if i.item_type == item.item_type && i.num + item.num < 100 => {
                    i.num += item.num;
                    return true;
                }
                _ => (),
            }
        }

        // Then just check for open slots
        for i_slot in self.items.iter_mut() {
            if i_slot.is_none() {
                *i_slot = Some(item);
                return true;
            }
        }

        return false;
    }

    /// Gets the item type of a given slot
    pub fn get_item_type(&self, slot: InventorySlotRef) -> Option<ItemType> {
        match slot {
            InventorySlotRef::Inventory(i) => self.items[i].map(|i| i.item_type),
            InventorySlotRef::Helmet => self.helmet.map(|i| i.item_type),
            InventorySlotRef::Body => self.body.map(|i| i.item_type),
            InventorySlotRef::Weapon => self.weapon.map(|i| i.item_type),
            InventorySlotRef::Ring => self.ring.map(|i| i.item_type),
        }
    }

    /// Takes an item from a slot, setting it to None and returning the item.
    pub fn take_item(&mut self, slot: InventorySlotRef) -> Option<InventoryItem> {
        let tmp = match slot {
            InventorySlotRef::Inventory(i) => self.items[i].clone(),
            InventorySlotRef::Helmet => self.helmet.clone(),
            InventorySlotRef::Body => self.body.clone(),
            InventorySlotRef::Weapon => self.weapon.clone(),
            InventorySlotRef::Ring => self.ring.clone(),
        };
        let _ = self.set_item(slot, None);
        tmp

    }

    /// Checks if the given item type can go in the given inventory slot. This
    /// DOESN'T have any regard for the current stack size, it only 'type
    /// checks' (i.e. only helmets in the helmet slot)
    pub fn can_place_item(&self, slot: InventorySlotRef,
                          item_type: ItemType) -> bool {
        match slot {
            InventorySlotRef::Inventory(_) => true,
            InventorySlotRef::Helmet =>
                item_type.equipment_type() == Some(EquipmentType::Helmet),
            InventorySlotRef::Body =>
                item_type.equipment_type() == Some(EquipmentType::Body),
            InventorySlotRef::Weapon =>
                item_type.equipment_type() == Some(EquipmentType::Weapon),
            InventorySlotRef::Ring =>
                item_type.equipment_type() == Some(EquipmentType::Ring),
        }
    }

    /// Overwrite the given slot with an item. Checks item types (for equipment
    /// slots).
    /// Returns the item that was overwritten (or None if there wasn't one)
    /// If typechecks failed, returns Err(()).
    pub fn set_item(&mut self, slot: InventorySlotRef,
                item: Option<InventoryItem>) -> Result<Option<InventoryItem>, ()> {
        if item.is_some() && !self.can_place_item(slot, item.unwrap().item_type) { return Err(()); }
        let tmp;
        match slot {
            InventorySlotRef::Inventory(ix) => {
                tmp = self.items[ix];
                self.items[ix] = item;
            },
            InventorySlotRef::Helmet => {
                tmp = self.helmet;
                self.helmet = item;
            }
            InventorySlotRef::Body => {
                tmp = self.body;
                self.body = item;
            }
            InventorySlotRef::Weapon => {
                tmp = self.weapon;
                self.weapon = item;
            }
            InventorySlotRef::Ring => {
                tmp = self.ring;
                self.ring = item;
            }
        }
        Ok(tmp)
    }
}
