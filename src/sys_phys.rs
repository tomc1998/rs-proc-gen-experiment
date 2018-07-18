//! Module for physics - movement & collision / resolution

use std::marker;
use specs::*;
use comp::*;
use std::mem;
use DeltaTime;

pub struct PhysSys<C0: Coll<C1>, C1: Coll<C0>> {
    m0: marker::PhantomData<C0>,
    m1: marker::PhantomData<C1>,
}

impl<C0: Coll<C1>, C1: Coll<C0>> PhysSys<C0, C1> {
    pub fn new() -> PhysSys<C0, C1> {
        PhysSys {
            m0: marker::PhantomData,
            m1: marker::PhantomData,
        }
    }
}

impl<'a, C0: Coll<C1> + Component, C1: Coll<C0> + Component> System<'a> for PhysSys<C0, C1> {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, C0>,
        ReadStorage<'a, C1>,
        Entities<'a>,
        WriteStorage<'a, Pos>);

    fn run(&mut self, (delta, vel_s, coll0_s, coll1_s, entities_s, mut pos_s): Self::SystemData) {
        use specs::Join;

        for (vel, pos) in (&vel_s, &mut pos_s).join() {
            pos.x += vel.x * delta.0;
            pos.y += vel.y * delta.0;
        }

        // Update entities that collide
        for (e0, pos0, coll0) in (&*entities_s, &pos_s, &coll0_s).join() {
            let flags = coll0.flags();
            if flags.0 & COLL_STATIC > 0 || flags.0 & COLL_SOLID == 0 { continue }
            // No broad phase, just brute force
            // TODO: Implement broad-phase collision
            for (e1, pos1, coll1) in (&*entities_s, &pos_s, &coll1_s).join() {
                if e1 == e0 { continue; }
                let res = coll0.resolve(coll1, pos0.to_vec(), pos1.to_vec());
                // Some bullshit transmuting to mutate pos
                unsafe {
                    let pos0_ptr : *mut Pos = mem::transmute(pos0);
                    if coll1.flags().0 & COLL_STATIC > 0 {
                        (*pos0_ptr).x += res.x;
                        (*pos0_ptr).y += res.y;
                    } else {
                        (*pos0_ptr).x += res.x / 2.0;
                        (*pos0_ptr).y += res.y / 2.0;
                    }
                }
            }
        };
    }
}
