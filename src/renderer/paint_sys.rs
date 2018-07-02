use super::*;

pub struct Painter;

#[derive(Clone, Debug)]
pub struct VertexBuffer {
    pub v_buf: Vec<Vertex>,
    pub size: u32,
}

use specs::*;
use comp::{Pos, DebugRender};
impl<'a> System<'a> for Painter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, DebugRender>);

    fn run(&mut self, (mut vertex_buffer, atlas, pos_s, dr_s): Self::SystemData) {
        use specs::Join;

        // Load debug data into v_buf & create a slice
        let white = atlas.rect_for_key(TextureKey::White).unwrap();
        let mut ix = 0;
        for (pos, dr) in (&pos_s, &dr_s).join() {
            Renderer::rect(&mut vertex_buffer.v_buf[ix * 6..(ix+1) * 6], &white,
                           pos.x - dr.w/2.0, pos.y - dr.h/2.0,
                           dr.w, dr.h, dr.col);
            ix += 1;
        }
        vertex_buffer.size = (ix * 6) as u32;

    }
}
