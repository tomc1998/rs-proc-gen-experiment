use specs::*;
use inventory::Inventory;
use comp::*;

/// Sets the player's equipment to match the inventory slots. This relies on the
/// PlayerControlled component (meaning that multiple player controlled NPCs
/// might fuck this up!
/// NOTE: This will just panic if there are multiple playercontrolled components
/// with equipment components.
pub struct SetEquipmentSys;

impl<'a> System<'a> for SetEquipmentSys {
    type SystemData = (
        ReadExpect<'a, Inventory>,
        ReadStorage<'a, PlayerControlled>,
        WriteStorage<'a, Equipment>);

    fn run(&mut self, (inventory, pc_s, mut eq_s) : Self::SystemData) {
        // First, assert that there aren't multiple playercontrolled entities
        // with equipment
        assert!((&pc_s, &eq_s).join().count() <= 1);

        // Now find the player and set their equipment to whatever's in the
        // inventory.
        if let Some((_, eq)) = (&pc_s, &mut eq_s).join().next() {
            if let Some(item) = inventory.helmet {
                eq.helmet = Some(item.item_type.as_helmet());
            } else { eq.helmet = None }
            if let Some(item) = inventory.body {
                eq.body = Some(item.item_type.as_body());
            } else { eq.body = None }
            if let Some(item) = inventory.weapon {
                eq.weapon = Some(item.item_type.as_weapon());
            } else { eq.weapon = None }
            if let Some(item) = inventory.ring {
                eq.ring = Some(item.item_type.as_ring());
            } else { eq.ring = None }
        }
    }
}
