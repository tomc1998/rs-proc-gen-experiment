use super::*;
use specs::*;
use comp::*;
use comp;
use {GameVertexBuffer, TerrainVertexBuffer, TerrainVertexBufferNeedsUpdate};
mod ui_inventory;

pub use self::ui_inventory::{
    InventoryPainter,
    NUM_COLUMNS as INVENTORY_NUM_COLUMNS,
    SLOT_SIZE as INVENTORY_SLOT_SIZE};

#[derive(Clone, Debug)]
pub struct VertexBuffer {
    pub v_buf: Vec<Vertex>,
    pub size: u32,
}

/// Paints components with AnimSprite or StaticSprite and Pos.
pub struct SpritePainter;

impl SpritePainter {
    /// Draw a sprite (origin is bottom center)
    /// # Params
    /// * upright - Render this sprite 'upright', i.e. spanning some depth.
    /// Ignored if rot is not 0.0.
    #[inline]
    fn draw_sprite<'a>(vertex_buffer: &mut VertexBuffer, ix: &mut usize,
                       // Components
                       pos: &Pos, e: Entity,
                       // Size
                       w: f32, h: f32,
                       // Additional systems
                       tint_s: &ReadStorage<'a, Tint>,
                       rot_s: &ReadStorage<'a, Rot>,
                       upright: bool,
                       // Tex
                       tex: &UvRect) {
        let col = if let Some(tint) = tint_s.get(e) {
            tint.col.clone()
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        let rot = if let Some(rot) = rot_s.get(e) { rot.angle } else { 0.0 };
        if rot != 0.0 {
            Renderer::rect_rot(&mut vertex_buffer.v_buf[*ix .. *ix+6],
                               tex,                          // UV
                               pos.pos.x - w/2.0, // X
                               pos.pos.y - h/2.0, // Y
                               -pos.z,         // Z
                               w, h, col, rot);
        } else if upright {
            Renderer::rect_upright(&mut vertex_buffer.v_buf[*ix .. *ix+6],
                           tex,                          // UV
                           pos.pos.x - w/2.0, // X
                           pos.pos.y,     // Y
                           -pos.z,         // Z
                           w, h, col);
        } else {
            Renderer::rect(&mut vertex_buffer.v_buf[*ix .. *ix+6],
                           tex,                          // UV
                           pos.pos.x - w/2.0, // X
                           pos.pos.y,     // Y
                           -pos.z,         // Z
                           w, h, col);
        }
        *ix += 6;
    }
}

impl<'a> System<'a> for SpritePainter {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameVertexBuffer>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Equipment>,
        ReadStorage<'a, Tint>,
        ReadStorage<'a, Rot>,
        ReadStorage<'a, comp::AnimSprite>,
        ReadStorage<'a, StaticSprite>);

    fn run(&mut self, (entities_s, mut vertex_buffer, atlas, pos_s,
                       equipment_s, tint_s, rot_s, anim_s, static_s):
           Self::SystemData) {
        use specs::Join;

        let vertex_buffer = &mut vertex_buffer.0;

        // Animated
        let mut ix = vertex_buffer.size as usize;
        for (e, pos, anim) in (&*entities_s, &pos_s, &anim_s).join() {
            let tex = atlas.rect_for_anim_sprite(anim.anim_key.clone()).unwrap()
                .frame(anim.anim, anim.curr_frame, &atlas.frame_set_map);
            SpritePainter::draw_sprite(
                vertex_buffer, &mut ix, &pos, e, anim.w, anim.h,
                &tint_s, &rot_s,
                anim.flags & ANIM_SPRITE_UPRIGHT > 0,
                &tex);
        }

        // Static
        for (e, pos, sprite) in (&*entities_s, &pos_s, &static_s).join() {
            let tex = atlas.rect_for_tex(sprite.sprite.clone()).unwrap();
            SpritePainter::draw_sprite(
                vertex_buffer, &mut ix, &pos, e, sprite.w, sprite.h, &tint_s, &rot_s,
                sprite.flags & STATIC_SPRITE_UPRIGHT > 0,
                &tex);
        }

        // Equipment
        for (e, pos, anim, equipment) in (&*entities_s, &pos_s, &anim_s,
                                          &equipment_s).join() {
            if let Some(ref equipment) = equipment.body {
                let tex = atlas.rect_for_anim_sprite(
                    equipment.get_equipment_anim()).unwrap()
                    .frame(anim.anim, anim.curr_frame, &atlas.frame_set_map);
                SpritePainter::draw_sprite(
                    vertex_buffer, &mut ix, &pos, e, anim.w, anim.h,
                    &tint_s, &rot_s, true, &tex);
            }
            if let Some(ref equipment) = equipment.helmet {
                let tex = atlas.rect_for_anim_sprite(
                    equipment.get_equipment_anim()).unwrap()
                    .frame(anim.anim, anim.curr_frame, &atlas.frame_set_map);
                SpritePainter::draw_sprite(
                    vertex_buffer, &mut ix, &pos, e, anim.w, anim.h,
                    &tint_s, &rot_s, true, &tex);
            }
        }

        vertex_buffer.size = ix as u32;
    }
}

/// Paints components with a Pos and Tilemap
pub struct TilemapPainter {
    /// If false will re-buffer tilemaps
    buffered: bool
}

impl TilemapPainter {
    pub fn new() -> TilemapPainter {
        TilemapPainter {
            buffered: false,
        }
    }
}

impl<'a> System<'a> for TilemapPainter {
    type SystemData = (
        WriteExpect<'a, TerrainVertexBuffer>,
        WriteExpect<'a, TerrainVertexBufferNeedsUpdate>,
        ReadExpect<'a, TextureAtlas<TextureKey>>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Tilemap>);

    fn run(&mut self, (mut vertex_buffer, mut needs_update, atlas, pos_s, tm_s): Self::SystemData) {
        if self.buffered { return }
        self.buffered = true;
        needs_update.0 = true;

        let vertex_buffer = &mut vertex_buffer.0;

        let mut ix = vertex_buffer.size as usize;
        for (pos, tm) in (&pos_s, &tm_s).join() {
            let tileset = atlas.rect_for_tileset(tm.tileset.convert_to_tex_key()).unwrap();
            for x in 0..TILEMAP_SIZE {
                for y in 0..TILEMAP_SIZE {
                    let x_pos = pos.pos.x * 32.0 * TILEMAP_SIZE as f32 + x as f32 * 32.0;
                    let y_pos = pos.pos.y * 32.0 * TILEMAP_SIZE as f32 + y as f32 * 32.0;
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
                                   x_pos, y_pos, 0.0, // X, Y, Z
                                   32.0, 32.0,            // W, H
                                   [1.0, 1.0, 1.0, 1.0]); // Col
                    ix += 6;
                }
            }
        }
        vertex_buffer.size = ix as u32;
    }
}
