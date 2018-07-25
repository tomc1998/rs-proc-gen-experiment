//! This module is about controlling the UI. For actual UI painting, see the
//! renderer::paint_sys::* modules.

mod inventory;

pub use self::inventory::{InventoryState, InventorySlotRef};

use inventory::Inventory;
use camera::Camera;
use specs::*;
use input::{*, self};

/// The state of the ui (i.e. is the inventory open?)
#[derive(Default, Clone, Debug)]
pub struct UIState {
    pub inventory_open: bool,
    /// This contains the state of the inventory. If the inventory is not open,
    /// consider this state meaningless.
    pub inventory_state: InventoryState,
}

/// System that listens to input and effects the UI state accordingly.
pub struct UIInputSystem;

impl<'a> System<'a> for UIInputSystem {
    type SystemData = (ReadExpect<'a, InputState>,
                       ReadExpect<'a, Camera>,
                       ReadExpect<'a, Inventory>,
                       Write<'a, UIState>);

    fn run(&mut self, (input_state, camera, inventory,
                       mut ui_state): Self::SystemData) {
        // Open / close some UIs
        if *input_state.pressed.get(&input::Command::ToggleInventory).unwrap() {
            ui_state.inventory_open = !ui_state.inventory_open;
        }

        // Process UIs that are open
        if ui_state.inventory_open {
            inventory::process_ui(&input_state, camera.w, camera.h,
                                  &inventory, &mut ui_state.inventory_state);
        }
    }
}
