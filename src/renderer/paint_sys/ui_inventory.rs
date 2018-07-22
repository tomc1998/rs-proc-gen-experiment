use ui::UIState;
use inventory::*;
use camera::Camera;
use specs::*;
use super::*;

pub struct InventoryPainter;

const NUM_COLUMNS : usize = 6;
const NUMBER_COLOR : [f32; 4] = [143.0 / 255.0,
                                 126.0 / 255.0,
                                 110.0 / 255.0, 1.0];

impl<'a> System<'a> for InventoryPainter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        Read<'a, UIState>,
        ReadExpect<'a, Inventory>,
        ReadExpect<'a, Camera>,
        ReadExpect<'a, TextureAtlas<TextureKey>>
    );

    fn run(&mut self, (mut vertex_buffer, ui_state, inventory, camera, atlas): Self::SystemData) {
        if !ui_state.inventory_open { return }

        let mut ix = vertex_buffer.size as usize;

        let inv_x = (camera.pos.x + camera.w / 2.0 - 300.0).to_f32();
        let inv_y = (camera.pos.y + camera.h / 2.0 - 200.0).to_f32();

        let tex = atlas.rect_for_tex(TextureKey::InventoryMockup).unwrap();
        // Draw mockup
        Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                       &tex,                  // UV
                       inv_x, inv_y, 1000.0, // X, Y, Z
                       600.0, 400.0, // W, H
                       [1.0, 1.0, 1.0, 1.0]); // Col
        ix += 6;

        // First, offset inv_x and inv_y so they're positioned at the first item position
        let inv_x = inv_x + 9.0 * 4.0;
        let inv_y = inv_y + 13.0 * 4.0;
        // Also find the offset for the numbers at the bottom
        let num_off_x = 3.0 * 4.0;
        let num_off_y = 17.0 * 4.0;
        // Get the number font
        let font = atlas.bitmap_font(TextureKey::FontTinyNumbers).unwrap();
        // Draw items
        for (inv_ix, item) in inventory.items.iter().enumerate() {
            if item.is_none() { continue }
            let item = item.unwrap();
            // Figure out the position to draw this icon at
            // times 4.0 because of pixel upscale
            let x = inv_x + (inv_ix % NUM_COLUMNS) as f32 * 24.0 * 4.0;
            let y = inv_y + (inv_ix / NUM_COLUMNS) as f32 * 26.0 * 4.0;
            let tex = atlas.rect_for_tex(item.item_type.get_icon_tex_key()).unwrap();
            // Draw icon
            Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                           &tex,                  // UV
                           x, y, 1000.0, // X, Y, Z
                           56.0, 56.0, // W, H
                           [1.0, 1.0, 1.0, 1.0]); // Col
            ix += 6;

            // Draw numbers
            let num0 = item.num / 10;
            let num1 = item.num % 10;
            let tex0 = font.rect_for_char(num0.to_string().chars().next().unwrap()).unwrap();
            let tex1 = font.rect_for_char(num1.to_string().chars().next().unwrap()).unwrap();
            Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                           &tex0, x + num_off_x + 2.0, y + num_off_y, 1000.0,
                           12.0, 20.0, NUMBER_COLOR);
            ix += 6;
            Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                           &tex1, x + num_off_x + 20.0 - 2.0, y + num_off_y, 1000.0,
                           12.0, 20.0, NUMBER_COLOR);
            ix += 6;
        }


        vertex_buffer.size = ix as u32;
    }
}

