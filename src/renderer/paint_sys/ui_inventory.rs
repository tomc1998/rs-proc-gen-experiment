use ui::UIState;
use camera::Camera;
use specs::*;
use super::*;

pub struct InventoryPainter;

impl<'a> System<'a> for InventoryPainter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        Read<'a, UIState>,
        ReadExpect<'a, Camera>,
        ReadExpect<'a, TextureAtlas<TextureKey>>
    );

    fn run(&mut self, (mut vertex_buffer, ui_state, camera, atlas): Self::SystemData) {
        if !ui_state.inventory_open { return }

        let mut ix = vertex_buffer.size as usize;

        let tex = atlas.rect_for_tex(TextureKey::InventoryMockup).unwrap();
        // Draw mockup
        Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                       &tex,                  // UV
                       (camera.w / 2.0 - 300.0).to_f32(),
                       (camera.h / 2.0 - 200.0).to_f32(), 1000.0, // X, Y, Z
                       600.0, 400.0, // W, H
                       [1.0, 1.0, 1.0, 1.0]); // Col

        ix += 6;

        vertex_buffer.size = ix as u32;
    }
}

