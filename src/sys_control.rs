//! Module for controller systems - either AI or input controlled

use input;
use specs::*;
use comp::{PlayerControlled, Vel};

pub struct PlayerControllerSys;

impl<'a> System<'a> for PlayerControllerSys {
    type SystemData = (
        Read<'a, input::InputState>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, PlayerControlled>);

    fn run(&mut self, (input_state, mut vel_s, pc_s): Self::SystemData) {
        use specs::Join;
        for (vel, pc) in (&mut vel_s, &pc_s).join() {
            if *input_state.down.get(&input::Command::MoveLeft).unwrap() {
                vel.x = -pc.move_speed;
            }
            else if *input_state.down.get(&input::Command::MoveRight).unwrap() {
                vel.x = pc.move_speed;
            }
            else {
                vel.x = 0.0;
            }
            if *input_state.down.get(&input::Command::MoveUp).unwrap() {
                vel.y = -pc.move_speed;
            }
            else if *input_state.down.get(&input::Command::MoveDown).unwrap() {
                vel.y = pc.move_speed;
            }
            else {
                vel.y = 0.0;
            }
        }
    }

}
