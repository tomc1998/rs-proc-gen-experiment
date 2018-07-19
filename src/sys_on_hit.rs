use DeltaTime;
use specs::*;
use comp::*;
use fpa::*;

pub struct KnockbackSys;

impl<'a> System<'a> for KnockbackSys {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, Knockback>,
        WriteStorage<'a, Pos>);

    fn run(&mut self, (entities_s, delta, mut knockback_s, mut pos_s): Self::SystemData) {
        let mut to_remove = Vec::new();
        for (e, knockback, pos) in (&*entities_s, &mut knockback_s, &mut pos_s).join() {
            pos.pos += knockback.knockback.to_32() * Fx32::new(delta.0);
            knockback.duration -= delta.0 * 1000.0;
            if knockback.duration.0 <= 0 {
                to_remove.push(e)
            }
        }
        for e in to_remove {
            knockback_s.remove(e);
        }
    }
}
