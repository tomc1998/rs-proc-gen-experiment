use specs::*;
use comp::*;

pub struct TrackPosSys;


impl<'a> System<'a> for TrackPosSys {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        ReadStorage<'a, TrackPos>);

    fn run(&mut self, (entities, mut pos_s, track_pos_s): Self::SystemData) {
        for (e, t) in (&*entities, &track_pos_s).join() {
            // First find the entity to track
            let pos;
            if entities.is_alive(t.e) {
                if let Some(t_pos) = pos_s.get(t.e) {
                    pos = t_pos.pos.clone();
                } else { continue; }
            } else { continue; }

            // Now track the position
            pos_s.get_mut(e).map(|p| p.pos = pos);
        }
    }
}
