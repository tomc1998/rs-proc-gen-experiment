//! Module for physics - movement & collision / resolution

use std::marker;
use specs::*;
use comp::*;
use DeltaTime;
use Collisions;
use CollisionMeta;
use fpa::*;
use fpavec::*;

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
        WriteExpect<'a, Collisions>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, C0>,
        ReadStorage<'a, C1>,
        Entities<'a>,
        WriteStorage<'a, Pos>,
    );

    fn run(&mut self, (delta, mut collisions, vel_s, coll0_s,
                       coll1_s, entities_s, mut pos_s): Self::SystemData) {
        use specs::Join;

        for (vel, pos) in (&vel_s, &mut pos_s).join() {
            pos.pos.x += vel.vel.x * delta.0;
            pos.pos.y += vel.vel.y * delta.0;
        }

        // Update entities that collide
       for (e0, coll0) in (&*entities_s, &coll0_s).join() {
            let flags0 = coll0.flags();
            // No broad phase, just brute force
            // TODO: Implement broad-phase collision
            let mut res = Vec16::zero();
            if let Some(pos0) = pos_s.get(e0) {
                for (e1, pos1, coll1) in (&*entities_s, &pos_s, &coll1_s).join() {
                    if e1 == e0 { continue; }
                    let this_res = coll0.resolve(coll1, pos0.pos, pos1.pos);
                    if this_res.len().0 == 0 { continue; }

                    // Insert into collisions
                    collisions.0.push((e0, e1, CollisionMeta {
                        normal: this_res.nor()
                    }));
                    let flags1 = coll1.flags();

                    // Process physics (or skip depending on the flags)
                    if flags0 & COLL_SOLID == 0 ||
                        flags0 & COLL_STATIC > 0 ||
                        flags1 & COLL_SOLID == 0 { continue }
                    if flags1 & COLL_STATIC > 0 {
                        res += this_res;
                    } else {
                        res += this_res / Fx16::new(2.0);
                    }
                }
            } else { continue }
            // Now add this res to e0's pos component
            let pos = pos_s.get_mut(e0).unwrap();
            pos.pos.x += res.x;
            pos.pos.y += res.y;
        };
    }
}
