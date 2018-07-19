use Collisions;
use DeltaTime;
use specs::*;
use comp::*;

pub struct HealthSys;

impl<'a> System<'a> for HealthSys {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, DeltaTime>,
        ReadExpect<'a, Collisions>,
        ReadStorage<'a, Hurt>,
        WriteStorage<'a, Health>);

    fn run(&mut self, (entities_s, delta, collisions, hurt_s, mut health_s): Self::SystemData) {

        for mut health in (&mut health_s).join() {
            if health.inv_time.0 > 0 {
                health.inv_time -= delta.0 * 1000.0;
                if health.inv_time.0 < 0 {
                    health.inv_time.0 = 0;
                }
            }
        }

        for (e0, e1) in &collisions.0 {
            // if e0 has health and e1 is a hurt, then hurt e0
            if let Some(health) = health_s.get_mut(*e0) {
                if health.inv_time.0 != 0 {continue} 
                if let Some(hurt) = hurt_s.get(*e1) {
                    if health.hurt(&hurt) {
                        entities_s.delete(*e0).unwrap();
                    }
                    health.inv_time = health.max_inv_time;
                }
            }
        }
    }
}
