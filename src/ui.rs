//! This module is about controlling the UI. For actual UI painting, see the
//! renderer::paint_sys::* modules.

use specs::*;
use input::{*, self};

/// The state of the ui (i.e. is the inventory open?)
#[derive(Default, Clone, Debug)]
pub struct UIState {
    pub inventory_open: bool,
}

/// System that listens to input and effects the UI state accordingly.
pub struct UIInputSystem;

impl<'a> System<'a> for UIInputSystem {
    type SystemData = (ReadExpect<'a, InputState>,
                       Write<'a, UIState>);

    fn run(&mut self, (input_state, mut ui_state): Self::SystemData) {
        if *input_state.pressed.get(&input::Command::ToggleInventory).unwrap() {
            ui_state.inventory_open = !ui_state.inventory_open;
        }
    }
}


