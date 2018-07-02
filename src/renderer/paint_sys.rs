use super::*;
use specs::*;
use comp::*;

#[derive(Clone, Debug)]
pub struct VertexBuffer {
    pub v_buf: Vec<Vertex>,
    pub size: u32,
}

/// Paints components with Pos and DebugRender.
pub struct DebugPainter;
impl<'a> System<'a> for DebugPainter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, DebugRender>);

    fn run(&mut self, (mut vertex_buffer, atlas, pos_s, dr_s): Self::SystemData) {
        use specs::Join;

        let white = atlas.rect_for_key(TextureKey::White).unwrap();
        let mut ix = vertex_buffer.size as usize;
        for (pos, dr) in (&pos_s, &dr_s).join() {
            Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6], &white,
                           pos.x - dr.w/2.0, pos.y - dr.h/2.0,
                           dr.w, dr.h, dr.col);
            ix += 6;
        }
        vertex_buffer.size = ix as u32;

    }
}

/// Paints components with a Pos and Tilemap
pub struct TilemapPainter;
impl<'a> System<'a> for TilemapPainter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Tilemap>);

    fn run(&mut self, (mut vertex_buffer, atlas, pos_s, tm_s): Self::SystemData) {
        use specs::Join;

        let mut ix = vertex_buffer.size as usize;
        for (pos, tm) in (&pos_s, &tm_s).join() {
            let tileset = atlas.rect_for_tileset(tm.tileset.convert_to_tex_key()).unwrap();
            ix += 6;
            for x in 0..TILEMAP_SIZE {
                for y in 0..TILEMAP_SIZE {
                    // Figure out the tile ix
                    let (tx, ty) = match tm.tileset {
                        TilesetEnum::Grass => match tm.data[x + y * TILEMAP_SIZE] {
                            0 => (4, 0), // Dirt
                            1 => (0, 0), // Grass
                            2 => (4, 4), // Water
                            t => panic!("Tile {} not found", t)
                        }
                    };
                    Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6], &tileset.tile(tx, ty),
                                   pos.x + x as f32 * 32.0, pos.y + y as f32 * 32.0, 32.0, 32.0,
                                   [1.0, 1.0, 1.0, 1.0]);
                    ix += 6;
                }
            }
        }
        vertex_buffer.size = ix as u32;


    }
}
