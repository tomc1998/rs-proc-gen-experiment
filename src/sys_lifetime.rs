use DeltaTime;
use specs::*;
use comp::*;

pub struct LifetimeSys;

impl<'a> System<'a> for LifetimeSys {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, Lifetime>,
        Entities<'a>);

    fn run(&mut self, (delta, mut lifetime_s, entities_s): Self::SystemData) {
        use specs::Join;

        for (lifetime, e) in (&mut lifetime_s, &*entities_s).join() {
            lifetime.lifetime -= delta.0 * 1000.0;
            if lifetime.lifetime.0 <= 0 {
                entities_s.delete(e).unwrap();
            }
        }
    }
}
