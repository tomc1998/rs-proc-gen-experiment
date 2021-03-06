//! Module for stepping animations

use specs::*;
use comp::{AnimSprite, ANIM_SPRITE_NO_LOOP};
use DeltaTime;

pub struct AnimSpriteSys;

impl<'a> System<'a> for AnimSpriteSys {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, AnimSprite>);

    fn run(&mut self, (delta, mut anim_s): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;
        (&mut anim_s).par_join().for_each(|anim| {
            anim.curr_frame_time += delta.0 * 1000.0;
            if anim.curr_frame_time > anim.frame_time {
                anim.curr_frame_time -= anim.frame_time;
                if !(anim.flags & ANIM_SPRITE_NO_LOOP > 0 && anim.curr_frame >= anim.num_frames - 1) {
                    anim.curr_frame = (anim.curr_frame + 1) % anim.num_frames;
                } else if anim.curr_frame < anim.num_frames - 1 {
                    anim.curr_frame = anim.curr_frame + 1;
                }
            }
        });
    }

}
