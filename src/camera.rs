use fpa::*;
use comp::*;
use specs::*;
use fpavec::*;

/// State for rendering. Passed to the render function.
fn gen_ortho_mat(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> [[f32; 4]; 4] {
    [[2.0/(r-l),       0.0,        0.0, -(r+l)/(r-l)],
     [0.0,       2.0/(t-b),        0.0, -(t+b)/(t-b)],
     [0.0,             0.0, -2.0/(f-n), -(f+n)/(f-n)],
     [0.0,             0.0,        0.0,          1.0]]
}

pub struct Camera {
    pub pos: Vec32,
    pub w: Fx32,
    pub h: Fx32,
}

impl Camera {
    pub fn new(w: f32, h: f32) -> Camera {
        Camera { pos: Vec32::zero(), w: Fx32::new(w), h: Fx32::new(h) }
    }
    pub fn gen_ortho_mat(&self) -> [[f32; 4]; 4] {
        gen_ortho_mat(self.pos.x.to_f32(), (self.pos.x + self.w).to_f32(),
                      self.pos.y.to_f32(), (self.pos.y + self.h).to_f32(),
                      -10000.0, 10000.0)
    }
}

/// System to make the camera follow an entity, should that entity exist
pub struct FollowCameraSys;

impl<'a> System<'a> for FollowCameraSys {
    type SystemData = (WriteExpect<'a, Camera>,
                       ReadStorage<'a, Pos>,
                       ReadStorage<'a, FollowCamera>);

    fn run(&mut self, (mut camera, pos_s, follow_camera_s): Self::SystemData) {
        if let Some((pos, _)) = (&pos_s, &follow_camera_s).join().next() {
            camera.pos = pos.pos - Vec32::new(camera.w/2.0, camera.h/2.0);
        }
    }
}

