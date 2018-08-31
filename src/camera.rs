use comp::*;
use input::*;
use specs::*;
use vec::*;
use cgmath;
use std::f32::consts::PI;

pub struct Camera {
    /// x / y position in space to look at
    pub pos: Vec32,
    /// Height of the camera above the plane
    pub height: f32,
    pub w: f32,
    pub h: f32,
    /// An angle for the camera in radians. Camera always points at the player, but can rotate around.
    pub rot: f32,
    /// Distance from the focal point of the camera
    pub dis: f32,
}

impl Camera {
    pub fn new(w: f32, h: f32) -> Camera {
        Camera { pos: Vec32::zero(), w: w, h: h,
                 height: 400.0,
                 rot: -PI / 2.0,
                 dis: 400.0,
        }
    }

    pub fn gen_ortho_proj_mat(&self) -> [[f32; 4]; 4] {
        cgmath::ortho(-self.w/2.0, self.w/2.0,
                      self.h/2.0, -self.h/2.0,
                      -10000.0, 10000.0).into()
    }
    pub fn gen_persp_proj_mat(&self) -> [[f32; 4]; 4] {
        (cgmath::perspective(cgmath::Deg(60.0), self.w / self.h, 0.1, 10000.0)
            * cgmath::Matrix4::from_nonuniform_scale(-1.0, 1.0, 1.0)).into()
    }

    /// Generate a view matrix from this camera, given a position to look at.
    pub fn gen_view_mat(&self) -> [[f32; 4]; 4] {
        // Calculate the camera pos
        let camera_pos = self.pos + Vec32::new((-self.rot).cos(), (-self.rot).sin()) * self.dis;
        // Lookat matrix
        let view = cgmath::Matrix4::look_at(
            cgmath::Point3::new(camera_pos.x, -self.height, camera_pos.y),
            cgmath::Point3::new(self.pos.x, 0.0, self.pos.y),
            cgmath::Vector3::new(0.0, -1.0, 0.0));
        view.into()
    }

    /// Generate a view matrix for the UI
    pub fn gen_ui_view_mat() -> [[f32; 4]; 4] {
        cgmath::Matrix4::look_at_dir(
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 1.0, 0.0),
            cgmath::Vector3::new(0.0, 0.0, 1.0)).into()
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
            camera.w = input_state.window_size.0 as f32;
            camera.h = input_state.window_size.1 as f32;

            // Update camera pos
            camera.pos = pos.pos;

            // Set world mouse position
            input_state.world_mouse = input_state.screen_mouse + camera.pos;
        }
    }
}

