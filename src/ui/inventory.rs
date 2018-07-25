use inventory::Inventory;
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
    /// A reference into the inventory. If this is Some, then the item at the
    /// current IX is being dragged / dropped.
    pub curr_drag_drop: Option<InventorySlotRef>,

    /// World pos of the mouse whilst drag / dropping
    pub drag_drop_world_pos: Vec32,

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
                  inventory: &Inventory,
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
    if let Some(_over) = inventory_state.curr_over {
        if *input_state.pressed.get(&Command::Primary).unwrap() {
            if inventory_state.curr_drag_drop.is_none() {
                inventory_state.curr_drag_drop = inventory_state.curr_over;
            } else {
                inventory_state.curr_drag_drop = None;
            }
        }
    }

    inventory_state.drag_drop_world_pos = input_state.world_mouse;
}
