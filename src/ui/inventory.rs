use inventory::*;
use vec::*;
use input::*;
use renderer::{INVENTORY_NUM_COLUMNS, INVENTORY_SLOT_SIZE};

/// A reference into the inventory. We need an enum becvause we could be
/// referring to either an equipment slot or an inventory slot.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum InventorySlotRef {
    Inventory(usize),
    Helmet,
    Body,
    Weapon,
    Ring,
}

/// Holds the state of the inventory UI (for drag/drop etc). Written to by the
/// update system and read from by the render system.
#[derive(Default, Clone, Debug)]
pub struct InventoryState {
    /// If this is Some, then we're drag / dropping this item.
    pub curr_drag_drop: Option<InventoryItem>,

    /// World pos of the mouse whilst drag / dropping
    pub drag_drop_pos: Vec32,

    /// A reference into the inventory. If this is Some, then the item at the
    /// current IX is being hovered over.
    /// Slight misnomer, we don't drag drop, we click to pickup and click to put
    /// down
    pub curr_over: Option<InventorySlotRef>,
}

/// Called by the UI sys if the inventory is open.
pub fn process_ui(input_state: &InputState,
                  camera_w: f32,
                  camera_h: f32,
                  inventory: &mut Inventory,
                  inventory_state: &mut InventoryState) {
    // Check all inventory slots for mouse hovering
    inventory_state.curr_over = None;
    for ix in 0..inventory.items.len() {
        // Figure out the x / y pos of this slot
        let x =
            // Inventory graphic pos on screen
            camera_w / 2.0 - 300.0
            // Pos of first inventory slot
            + 9.0 * 4.0
            // Inventory index offset
            + (ix % INVENTORY_NUM_COLUMNS) as f32 * 24.0 * 4.0;
        let y =
            camera_h / 2.0 - 300.0
            + 23.0 * 4.0
            + (ix / INVENTORY_NUM_COLUMNS) as f32 * 26.0 * 4.0;
        if input_state.is_screen_mouse_in_rect(
            x, y, INVENTORY_SLOT_SIZE, INVENTORY_SLOT_SIZE) {
            inventory_state.curr_over = Some(InventorySlotRef::Inventory(ix));
            break;
        }
    }

    // Check equipment slots for hovering
    let position_iter = [
        (17.0 * 4.0,  113.0 * 4.0, InventorySlotRef::Helmet),
        (49.0 * 4.0,  113.0 * 4.0, InventorySlotRef::Body),
        (81.0 * 4.0,  113.0 * 4.0, InventorySlotRef::Weapon),
        (113.0 * 4.0, 113.0 * 4.0, InventorySlotRef::Ring)].into_iter();
    for (x, y, slot) in position_iter {
        let x = camera_w / 2.0 - 300.0 + x;
        let y = camera_h / 2.0 - 300.0 + y;
        if input_state.is_screen_mouse_in_rect(
            x, y, INVENTORY_SLOT_SIZE, INVENTORY_SLOT_SIZE) {
            inventory_state.curr_over = Some(*slot);
            break;
        }
    }

    // Check mouse press pickups
    if let Some(over) = inventory_state.curr_over {
        if *input_state.pressed.get(&Command::Primary).unwrap() {
            // Only pickup if there's something in the slot
            if inventory_state.curr_drag_drop.is_none() &&
                inventory.get_item_type(over).is_some() {
                    inventory_state.curr_drag_drop =
                        inventory.take_item(over);
                } else if let Ok(item) = inventory.set_item(
                    over, inventory_state.curr_drag_drop) {
                    // Swap this inventory slot with whatever we're carrying, if
                    // we can place it here!
                    inventory_state.curr_drag_drop = item;
                }
        }
    }

    // Copy over the mouse pos for use in the UI
    inventory_state.drag_drop_pos = input_state.screen_mouse -
        Vec32::new(camera_w, camera_h) / 2.0
}
