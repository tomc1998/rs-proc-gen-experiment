use item::ItemType;

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
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Box::new([None; INVENTORY_SIZE])
        }
    }
}
