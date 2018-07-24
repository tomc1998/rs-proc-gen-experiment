use specs::*;
use comp::*;

/// Matches up animations (see comp::MatchAnim)
pub struct MatchAnimSys;

impl<'a> System<'a> for MatchAnimSys {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, MatchAnim>,
        WriteStorage<'a, AnimSprite>);
    fn run(&mut self, (entities, match_anim_s, mut anim_sprite_s): Self::SystemData) {
        for (e, match_anim) in (&*entities, &match_anim_s).join() {
            // First, find the animation to match to
            let (anim, curr_frame, num_frames);
            {
                if !entities.is_alive(e) { continue }
                if let Some(a) = anim_sprite_s.get(match_anim.e) {
                    anim = a.anim;
                    curr_frame = a.curr_frame;
                    num_frames = a.num_frames;
                } else { continue }
            }

            // Now lookup the animsprite component and add to it.
            if let Some(a) = anim_sprite_s.get_mut(e) {
                a.curr_frame_time = 0.0;
                a.anim = anim;
                a.curr_frame = curr_frame;
                a.num_frames = num_frames;
            }
        }
    }
}
