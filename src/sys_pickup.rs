//! System for dealing with picking up items

const MAGNETISM_SPEED : Fx32 = Fx32(30 * FPA_MUL as i32);
/// Damping applied each frame
const PICKUP_DAMPING : Fx32 = Fx32(10 * FPA_MUL as i32);

use inventory::Inventory;
use Collisions;
use specs::*;
use comp::*;
use fpa::*;

pub struct PickupSys;


impl<'a> System<'a> for PickupSys {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Collisions>,
        WriteExpect<'a, Inventory>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Pickup>,
        ReadStorage<'a, Collector>);

    fn run(&mut self, (entities_s, collisions, mut inventory, pos_s, mut vel_s, pickup_s,
                       collector_s): Self::SystemData) {
        // Check for collisions
        for (e0, e1, _) in &collisions.0 {
            // if e0 has collector and e1 has pickup, then pick e1 up
            match (collector_s.get(*e0), pickup_s.get(*e1)) {
                (Some(_collector), Some(pickup)) => {
                    if inventory.add_item(pickup.item) {
                        // Remove the pickup item
                        entities_s.delete(*e1).unwrap();
                    }
                }
                _ => ()
            }
        }

        // Apply 'magnetism'
        // Loop over all pickups and apply vel if possible
        for (_, p_pos, vel) in (&pickup_s, &pos_s, &mut vel_s).join() {
            // apply pickup damping
            if vel.vel.x.0 != 0 || vel.vel.y.0 != 0 {
                let vel_len = vel.vel.len();
                vel.vel *= (vel_len - PICKUP_DAMPING) / vel_len;
            }
            // Loop over all collectors
            for (collector, c_pos) in (&collector_s, &pos_s).join() {
                // Check if in range
                let vec = p_pos.pos - c_pos.pos;
                let dis = vec.len();
                if (dis - collector.magnet_radius.to_fx32()).0 < 0 {
                    // Apply vel
                    vel.vel += (-vec / dis) * MAGNETISM_SPEED;
                }
            }
        }
    }
}
