use super::*;
use specs::*;
use comp::*;
use comp;

#[derive(Clone, Debug)]
pub struct VertexBuffer {
    pub v_buf: Vec<Vertex>,
    pub size: u32,
}

/// Paints components with AnimSprite and Pos.
pub struct StaticSpritePainter;
impl<'a> System<'a> for StaticSpritePainter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, comp::StaticSprite>);

    fn run(&mut self, (mut vertex_buffer, atlas, pos_s, sprite_s): Self::SystemData) {
        use specs::Join;

        let mut ix = vertex_buffer.size as usize;
        for (pos, sprite) in (&pos_s, &sprite_s).join() {
            let tex = atlas.rect_for_tex(sprite.sprite.clone()).unwrap();
            Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                           &tex,                  // UV
                           (pos.pos.x - sprite.w/2.0).to_f32(),  // X
                           (pos.pos.y - sprite.h).to_f32(),      // Y
                           (pos.pos.y).to_f32(),                 // Z
                           sprite.w, sprite.h,    // W, H
                           [1.0, 1.0, 1.0, 1.0]); // Col
            ix += 6;
        }
        vertex_buffer.size = ix as u32;

    }
}

/// Paints components with AnimSprite and Pos.
pub struct AnimSpritePainter;
impl<'a> System<'a> for AnimSpritePainter {
    type SystemData = (
        WriteExpect<'a, VertexBuffer>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, comp::AnimSprite>);

    fn run(&mut self, (mut vertex_buffer, atlas, pos_s, anim_s): Self::SystemData) {
        use specs::Join;

        let mut ix = vertex_buffer.size as usize;
        for (pos, anim) in (&pos_s, &anim_s).join() {
            let tex = atlas.rect_for_anim_sprite(anim.anim.clone()).unwrap().frame(anim.curr_frame);
            Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                           &tex,                  // UV
                           (pos.pos.x - anim.w/2.0).to_f32(),    // X
                           (pos.pos.y - anim.h).to_f32(),        // Y
                           (pos.pos.y).to_f32(),                 // Z
                           anim.w, anim.h,        // W, H
                           [1.0, 1.0, 1.0, 1.0]); // Col
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
        ReadExpect<'a, Camera>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Tilemap>);

    fn run(&mut self, (mut vertex_buffer, camera, atlas, pos_s, tm_s): Self::SystemData) {
        use specs::Join;

        let mut ix = vertex_buffer.size as usize;
        for (pos, tm) in (&pos_s, &tm_s).join() {
            let tileset = atlas.rect_for_tileset(tm.tileset.convert_to_tex_key()).unwrap();
            ix += 6;
            for x in 0..TILEMAP_SIZE {
                for y in 0..TILEMAP_SIZE {
                    // Camera frustum cull.
                    // TODO: This is slower than it could be, even though branch
                    // prediction for something like this should be pretty fast
                    // - we should just loop over the tiles that we need to
                    // draw.
                    let x_pos = pos.pos.x * 32.0 * TILEMAP_SIZE as f32 + x as f32 * 32.0;
                    let y_pos = pos.pos.y * 32.0 * TILEMAP_SIZE as f32 + y as f32 * 32.0;
                    if x_pos + 32.0 < camera.x || x_pos > camera.x + camera.w ||
                        y_pos + 32.0 < camera.y || y_pos > camera.y + camera.h {
                            continue;
                        }
                    // Figure out the tile ix
                    let (tx, ty) = match tm.tileset {
                        TilesetEnum::Grass => match tm.data[x + y * TILEMAP_SIZE] {
                            0 => (5, 1), // Dirt
                            1 => (1, 1), // Grass
                            2 => (5, 5), // Water
                            t => panic!("Tile {} not found", t)
                        }
                    };
                    Renderer::rect(&mut vertex_buffer.v_buf[ix .. ix+6],
                                   &tileset.tile(tx, ty), // UV
                                   x_pos.to_f32(), y_pos.to_f32(), -2000.0, // X, Y, Z
                                   32.0, 32.0,            // W, H
                                   [1.0, 1.0, 1.0, 1.0]); // Col
                    ix += 6;
                }
            }
        }
        vertex_buffer.size = ix as u32;
    }
}
