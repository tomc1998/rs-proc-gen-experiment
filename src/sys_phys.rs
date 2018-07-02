//! Module for physics - movement & collision / resolution

use specs::*;
use comp::{Vel, Pos};

pub struct PhysSys;

pub struct DeltaTime(pub f32);

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
