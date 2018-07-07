//! Module for physics - movement & collision / resolution

use specs::*;
use comp::{Vel, Pos};
use DeltaTime;

pub struct PhysSys;

impl<'a> System<'a> for PhysSys {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, Vel>,
        WriteStorage<'a, Pos>);

    fn run(&mut self, (delta, vel_s, mut pos_s): Self::SystemData) {
        use specs::Join;
        for (vel, pos) in (&vel_s, &mut pos_s).join() {
            pos.x += vel.x * delta.0;
            pos.y += vel.y * delta.0;
        }
    }

}
