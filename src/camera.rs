use fpa::*;
use comp::*;
use input::*;
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
                       WriteExpect<'a, InputState>,
                       ReadStorage<'a, Pos>,
                       ReadStorage<'a, FollowCamera>);

    fn run(&mut self, (mut camera, mut input_state, pos_s, follow_camera_s): Self::SystemData) {
        if let Some((pos, _)) = (&pos_s, &follow_camera_s).join().next() {
            // Update the camera size depending on view size
            camera.w = Fx32::new(input_state.window_size.0 as f32);
            camera.h = Fx32::new(input_state.window_size.1 as f32);

            // Update camera pos
            camera.pos = pos.pos - Vec32::new(camera.w/2.0, camera.h/2.0);

            // Set world mouse position
            input_state.world_mouse = input_state.screen_mouse + camera.pos;

            // Additional translation for mouse pos. Also update mouse pos in input.
            // First, find the offset from the screen mouse-pos and the centre of the screen.
            let screen_m = input_state.screen_mouse - Vec32::new(camera.w/2.0, camera.h/2.0);

            // Apply additional translation
            camera.pos += screen_m * Fx32::new(0.25);
        }
    }
}

