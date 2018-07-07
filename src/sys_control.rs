//! Module for controller systems - either AI or input controlled

use input;
use specs::*;
use comp::{PlayerControlled, Vel, AnimSprite};
use renderer::TextureKey;

pub struct PlayerControllerSys;

impl<'a> System<'a> for PlayerControllerSys {
    type SystemData = (
        Read<'a, input::InputState>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, AnimSprite>,
        ReadStorage<'a, PlayerControlled>);

    fn run(&mut self, (input_state, mut vel_s, mut anim_s, pc_s): Self::SystemData) {
        use specs::Join;
        for (vel, anim, pc) in (&mut vel_s, &mut anim_s, &pc_s).join() {
            let mut anim_change = None;
            if *input_state.down.get(&input::Command::MoveUp).unwrap() {
                vel.y = -pc.move_speed;
                anim_change = Some(TextureKey::Human00WalkUp);
            }
            else if *input_state.down.get(&input::Command::MoveDown).unwrap() {
                vel.y = pc.move_speed;
                anim_change = Some(TextureKey::Human00WalkDown);
            }
            else {
                vel.y = 0.0;
            }
            if *input_state.down.get(&input::Command::MoveLeft).unwrap() {
                vel.x = -pc.move_speed;
                anim_change = Some(TextureKey::Human00WalkLeft);
            }
            else if *input_state.down.get(&input::Command::MoveRight).unwrap() {
                vel.x = pc.move_speed;
                anim_change = Some(TextureKey::Human00WalkRight);
            }
            else {
                vel.x = 0.0;
            }
            if let Some(anim_change) = anim_change {
                anim.set_anim(anim_change, 4, 150.0);
            } else if vel.x == 0.0 && vel.y == 0.0 {
                match anim.anim {
                    TextureKey::Human00WalkLeft =>
                        anim.set_anim(TextureKey::Human00IdleLeft, 1, 1000.0),
                    TextureKey::Human00WalkRight =>
                        anim.set_anim(TextureKey::Human00IdleRight, 1, 1000.0),
                    TextureKey::Human00WalkUp =>
                        anim.set_anim(TextureKey::Human00IdleUp, 1, 1000.0),
                    TextureKey::Human00WalkDown =>
                        anim.set_anim(TextureKey::Human00IdleDown, 1, 1000.0),
                    _ => ()
                }
            }
        }
    }

}
