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
        ReadStorage<'a, HurtKnockbackDir>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Tint>,
        WriteStorage<'a, Knockback>);

    fn run(&mut self, (entities_s, delta, collisions, hurt_s, hurt_knockback_dir_s,
                       mut health_s, mut tint_s, mut on_hit_s): Self::SystemData) {

        for (e, mut health) in (&*entities_s, &mut health_s).join() {
            if health.inv_time.0 > 0 {
                health.inv_time -= delta.0 * 1000.0;
                if health.inv_time.0 < 0 {
                    health.inv_time.0 = 0;
                    tint_s.remove(e);
                }
            }
        }

        for (e0, e1, _) in &collisions.0 {
            // if e0 has health and e1 has a hurt, then hurt e0
            if let Some(health) = health_s.get_mut(*e0) {
                if health.inv_time.0 != 0 {continue}
                if let Some(hurt) = hurt_s.get(*e1) {
                    if !health.mask.collides(&hurt.mask) { continue; }
                    if health.hurt(&hurt) {
                        entities_s.delete(*e0).unwrap();
                    }
                    health.inv_time = health.max_inv_time;
                    // Apply tint to e0
                    tint_s.insert(*e0, Tint {
                        col: [1.0, 0.1, 0.1, 1.0],
                    }).unwrap();
                    if let Some(kb) = hurt_knockback_dir_s.get(*e1) {
                        // Apply knockback to e0
                        on_hit_s.insert(*e0, Knockback {
                            knockback: kb.knockback,
                            duration: kb.duration,
                        }).unwrap();
                    }
                }
            }
        }
    }
}
