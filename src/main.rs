#![feature(test)]

#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate rand;
extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate image;
extern crate rusttype;
extern crate specs;
extern crate rayon;
#[macro_use] extern crate specs_derive;
extern crate num_integer;

#[cfg(test)]
extern crate test;

mod renderer;
mod comp;
mod input;
mod sys_control;
mod sys_health;
mod sys_phys;
mod sys_anim;
mod sys_lifetime;
mod sys_on_hit;
mod sys_pickup;
mod sys_death_drop;
mod sys_track_pos;
mod sys_match_anim;
mod vec;
mod ui;
mod camera;
mod math_util;
mod item;
mod inventory;
mod drop_tables;
mod equipment;

use comp::*;
use vec::*;
use specs::*;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlRequest, GlContext};
use glutin::Api::OpenGl;
use std::time;
use std::thread;
use rand::SeedableRng;

pub struct CollisionMeta {
    /// This normal points outwards from entity B to entity A (and is also used
    /// to resolve circ - circ collisions)
    /// Will be normalised.
    #[allow(dead_code)]
    normal: Vec32,
}

/// Lists pairs of collisions.
pub struct Collisions(Vec<(Entity, Entity, CollisionMeta)>);

pub struct DeltaTime(pub f32);

/// Entities that have been 'killed' and need to produce on-death effects. This
/// doesn't mean all deleted entities - it means alive characters have been
/// killed by combat or other effects.
pub struct KilledEntities(Vec<Entity>);

/// Empty specs::System to use in the dispatcher as a combiner for system
/// dependencies.
pub struct MarkerSys;
impl<'a> System<'a> for MarkerSys {
    type SystemData = ();
    fn run(&mut self, (): Self::SystemData) {}
}

/// Create the world and register all the components
fn create_world() -> specs::World {
    let mut world = specs::World::new();
    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<PlayerControlled>();
    world.register::<Tilemap>();
    world.register::<AnimSprite>();
    world.register::<StaticSprite>();
    world.register::<CollCircle>();
    world.register::<AISlime>();
    world.register::<Hurt>();
    world.register::<Health>();
    world.register::<Lifetime>();
    world.register::<Knockback>();
    world.register::<HurtKnockbackDir>();
    world.register::<Tint>();
    world.register::<Rot>();
    world.register::<Alliance>();
    world.register::<FollowCamera>();
    world.register::<Pickup>();
    world.register::<Collector>();
    world.register::<OnDeathDrop>();
    world.register::<TrackPos>();
    world.register::<MatchAnim>();
    world.register::<Equipment>();
    world
}

fn main() {
    // Create the window
    let mut events_loop = glutin::EventsLoop::new();
    let windowbuilder = glutin::WindowBuilder::new()
        .with_title("Triangle Example".to_string())
        .with_dimensions(512, 512);
    let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3, 3)));
    let (window, mut device, mut factory, color_view, depth_view) =
        gfx_glutin::init::<renderer::ColorFormat, renderer::DepthFormat>(
            windowbuilder, contextbuilder, &events_loop);

    let (w, h) = window.get_inner_size().unwrap();
    let (mut renderer, atlas) = renderer::Renderer::new(
        &mut factory, color_view, depth_view, Default::default());
    let camera = camera::Camera::new(w as f32, h as f32);

    // Create the ECS world, and a test entity, plus trees
    let mut world = create_world();
    use specs::Builder;
    // Player
    world.create_entity()
        .with(Pos { pos: Vec32::new(32.0, 32.0) })
        .with(Vel { vel: Vec32::zero() })
        .with(Alliance::good())
        .with(PlayerControlled::new())
        .with(FollowCamera)
        .with(Health::new(8, Hitmask(HITMASK_PLAYER)))
        .with(Collector { magnet_radius: 64.0 })
        .with(Equipment {
            head: Some(equipment::Helmet::BronzeHelmet),
            .. Default::default()
        })
        .with(CollCircle { r: 8.0, off: Vec32::zero(),
                           flags: COLL_SOLID})
        .with(AnimSprite::new(32.0, 32.0, 100.0,
                              4, renderer::TextureKey::Human00Anim))
        .build();
    // Tree
    world.create_entity()
        .with(Pos { pos: Vec32::new(100.0, 100.0) })
        .with(CollCircle { r: 12.0, off: Vec32::zero(),
                           flags: COLL_SOLID | COLL_STATIC})
        .with(StaticSprite { w: 64.0, h: 128.0,
                             sprite: renderer::TextureKey::GreenTree00})
        .build();
    // Slime
    world.create_entity()
        .with(Pos { pos: Vec32::new(200.0, 200.0) })
        .with(Vel { vel: Vec32::zero() })
        .with(Health::new(4, Hitmask(HITMASK_ENEMY)))
        .with(Hurt { damage: 2,
                     mask: Hitmask::default_enemy_attack(),
                     flags: 0 })
        .with(Alliance::evil())
        .with(OnDeathDrop {
            drop_table: drop_tables::DropTableKey::Slime,
            min_drops: 1,
            max_drops: 3,
        })
        .with(AISlime { move_target: Vec32::new(200.0, 200.0),
                        attack_target: None,
                        charge_time: 0.0,
                        state: SlimeState::Idle })
        .with(CollCircle { r: 8.0, off: Vec32::zero(), flags: COLL_SOLID})
        .with(AnimSprite::new(32.0, 32.0, 100000.0,
                              1, renderer::TextureKey::SlimeAnim))
        .build();

    // Create tilemaps
    for x in 0..10 {
        for y in 0..10 {
            world.create_entity()
                .with(Pos { pos: Vec32::new(x as f32, y as f32) })
                .with(Tilemap { tileset: TilesetEnum::Grass,
                                data: [1u8; TILEMAP_SIZE * TILEMAP_SIZE] })
                .build();
        }
    }

    // Create test inventory
    let mut inventory = inventory::Inventory::new();
    inventory.items[0] = Some(inventory::InventoryItem::new(item::ItemType::Money, 10));

    let input_map = input::InputMap::new();
    // Allocate cpu side v_buf
    let v_buf = vec![Default::default(); renderer::V_BUF_SIZE];
    world.add_resource(atlas);
    world.add_resource(camera);
    world.add_resource(DeltaTime(0.016));
    world.add_resource(Collisions(Vec::with_capacity(128)));
    world.add_resource::<ui::UIState>(Default::default());
    world.add_resource(input::InputState::new());
    world.add_resource(drop_tables::DropTableMap::new_standard_map());
    world.add_resource(inventory);
    world.add_resource(KilledEntities(Vec::new()));
    world.add_resource(renderer::VertexBuffer {
        v_buf: v_buf, size: 0,
    });

    // Build dispatcher
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(sys_lifetime::LifetimeSys, "lifetime", &[])
        // Control
        .with(ui::UIInputSystem, "ui_input", &[])
        .with(sys_control::PlayerControllerSys, "player_controller", &[])
        .with(sys_control::SlimeAISys, "slime_ai", &[])
        .with(MarkerSys, "control", &["player_controller", "slime_ai", "ui_input"])

        // Animation
        .with(sys_anim::AnimSpriteSys, "anim_sprite", &["control"])

        // Physics
        .with(sys_phys::PhysSys::<CollCircle, CollCircle>::new(), "phys_circ_circ", &["player_controller"])
        .with(MarkerSys, "phys", &["phys_circ_circ"])

        .with(sys_track_pos::TrackPosSys, "track_pos", &["phys"])
        .with(sys_match_anim::MatchAnimSys, "match_anim", &["phys"])

        // Camera control
        .with(camera::FollowCameraSys, "follow_camera", &["phys"])

        // Pickups
        .with(sys_pickup::PickupSys, "pickup", &["phys"])

        // Combat
        .with(sys_health::HealthSys, "health", &["phys"])
        .with(sys_on_hit::KnockbackSys, "oh_knockback", &["health"])

        .with(MarkerSys, "update",
              &["phys", "anim_sprite", "health", "follow_camera",
                "oh_knockback", "track_pos", "match_anim"])

        // After-death effects
        .with(sys_death_drop::OnDeathDropSys::new(
            rand::rngs::StdRng::from_rng(
                rand::thread_rng()).unwrap()),
              "on_death_drop", &["update"])

        // Paint
        .with(renderer::TilemapPainter, "tilemap_paint", &["update"])
        .with(renderer::SpritePainter, "sprite_paint", &["update"])
        .with(renderer::InventoryPainter, "ui_inventory_paint", &["update"])
        .build();

    dispatcher.setup(&mut world.res);

    // Number of frames until we print another frame time
    let mut fps_count_timer = 60;
    loop {
        let start = time::Instant::now();

        // update input
        {
            let mut input_state = world.write_resource::<input::InputState>();
            input_state.process_input(&input_map, &mut events_loop);
            if input_state.should_close { break; } // Early return for speedy exit
            // Update window size if needed
            if input_state.window_dimensions_need_update {
                println!("Resizing window viewport");
                renderer.update_window_size(&window);
            }
        }

        // Update & paint the world
        {
            dispatcher.dispatch_seq(&mut world.res);
            let mut v_buf = world.write_resource::<renderer::VertexBuffer>();
            renderer.clear();
            renderer.flush_render(&mut device, &v_buf, &world.read_resource::<camera::Camera>());
            window.swap_buffers().unwrap();
            device.cleanup();

            // Reset ECS state after rendering
            // After painting, we need to clear the v_buf
            v_buf.size = 0;
            // Clear collision list for next frame
            let mut collisions = world.write_resource::<Collisions>();
            collisions.0.clear();
            let mut killed = world.write_resource::<KilledEntities>();
            killed.0.clear();
        }

        // Actually delete all entities that need to be deleted
        world.maintain();

        // Calculate frame time
        let elapsed = start.elapsed();
        if fps_count_timer <= 0 {
            println!("Time taken (millis): {:?}",
                     elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64);
            fps_count_timer = 60;
        }
        fps_count_timer -= 1;
        // Sleep until we hit 60fps. Vsync works until the window isn't being
        // rendered, then we just consume CPU!
        if elapsed.subsec_millis() < 17 && elapsed.as_secs() == 0 {
            thread::sleep(time::Duration::from_millis(17) - elapsed);
        }

    }
}
