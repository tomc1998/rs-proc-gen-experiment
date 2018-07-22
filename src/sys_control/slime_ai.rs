use DeltaTime;
use specs::*;
use comp::*;
use vec::*;

const JUMP_SPEED : f32 = 900.0;

pub struct SlimeAISys;

impl<'a> System<'a> for SlimeAISys {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Alliance>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, AISlime>,
        WriteStorage<'a, AnimSprite>,
    );

    fn run(&mut self, (entities, delta, pos_s, alliance_s,
                       mut vel_s, mut ai_s, mut anim_s): Self::SystemData) {
        use specs::Join;
        for (e, pos, alliance, vel, ai, _anim) in (&*entities, &pos_s, &alliance_s,
                                                  &mut vel_s, &mut ai_s, &mut anim_s).join() {
            if ai.attack_target.is_none() {
                // Find targets
                for (target_e, target_pos, target_alliance) in (&*entities, &pos_s, &alliance_s).join() {
                    if e == target_e { continue }
                    if alliance.alliance.attacks(&target_alliance.alliance) {
                        // Check if in range (200 units)
                        if (target_pos.pos - pos.pos).len() - 200.0 < 0.0 {
                            ai.attack_target = Some(target_e);
                            break;
                        }
                    }
                }
                // If still none, just move around a bit
                if ai.attack_target.is_none() {
                }
            }
            else if ai.attack_target.is_some() {
                let target_pos = pos_s.get(ai.attack_target.unwrap());
                if target_pos.is_none() {
                    ai.attack_target = None;
                    vel.vel = Vec32::zero();
                    continue;
                }
                let target_pos = target_pos.unwrap();
                // Check if the target is gone (over 300 units)
                if (target_pos.pos - pos.pos).len() - 300.0 > 0.0 {
                    ai.attack_target = None;
                    vel.vel = Vec32::zero();
                    continue;
                }
                match ai.state {
                    SlimeState::Idle => {
                        ai.state = SlimeState::Charging;
                        ai.charge_time = 400.0;
                    }
                    SlimeState::Charging => {
                        ai.charge_time -= delta.0 * 1000.0;
                        if ai.charge_time < 0.0 {
                            ai.charge_time = 1200.0;
                            ai.state = SlimeState::Jumping;
                            // Set the velocity to jump towards the target
                            if target_pos.pos == pos.pos {
                                // Just jump without setting vel if we're on top of the target
                                continue;
                            }
                            let jump_dir = (target_pos.pos - pos.pos).nor();
                            vel.vel = jump_dir * JUMP_SPEED;
                        }
                    }
                    SlimeState::Jumping => {
                        // Damp vel & decrease charge time
                        vel.vel *= 0.9;
                        let len = vel.vel.len();
                        if len != 0.0 { vel.vel *= (len - 1.0) / len; }
                        ai.charge_time -= delta.0 * 1000.0;
                        if ai.charge_time < 0.0 {
                            vel.vel = Vec32::zero();
                            ai.state = SlimeState::Idle;
                        }
                    }
                }
            }
        }
    }
}
