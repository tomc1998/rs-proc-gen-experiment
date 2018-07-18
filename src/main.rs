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
mod sys_phys;
mod sys_anim;
mod fpa;
mod fpavec;

use comp::*;
use fpa::*;
use fpavec::*;
use specs::*;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlRequest, GlContext};
use glutin::Api::OpenGl;
use std::time;

pub struct DeltaTime(pub f32);

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
        &mut factory, color_view, depth_view, w, h, Default::default());
    let camera = renderer::Camera::new(w as f32, h as f32);

    // Create the ECS world, and a test entity, plus trees
    let mut world = create_world();
    use specs::Builder;
    // Player
    world.create_entity()
        .with(Pos { x: Fx32::new(32.0), y: Fx32::new(32.0) })
        .with(Vel { x: Fx16::new(0.0), y: Fx16::new(0.0) })
        .with(PlayerControlled { move_speed: Fx16::new(100.0) })
        .with(CollCircle { r: Fx16::new(16.0),
                           off: Vec16::new(Fx16::new(0.0), Fx16::new(0.0)),
                           flags: CollFlags(COLL_SOLID)})
        .with(AnimSprite { w: 32.0, h: 32.0,
                           curr_frame: 0, frame_time: Fx32::new(100.0),
                           curr_frame_time: Fx32::new(0.0),
                           num_frames: 4,
                           anim: renderer::TextureKey::Human00WalkLeft});
    // Tree
    world.create_entity()
        .with(Pos { x: Fx32::new(100.0), y: Fx32::new(100.0) })
        .with(CollCircle { r: Fx16::new(16.0),
                           off: Vec16::new(Fx16::new(0.0), Fx16::new(0.0)),
                           flags: CollFlags(COLL_SOLID | COLL_STATIC)})
        .with(StaticSprite { w: 64.0, h: 128.0,
                                 sprite: renderer::TextureKey::GreenTree00});

    // Create tilemaps
    for x in 0..10 {
        for y in 0..10 {
            world.create_entity()
                .with(Pos { x: Fx32::new(x as f32), y: Fx32::new(y as f32) })
                .with(Tilemap { tileset: TilesetEnum::Grass,
                                      data: [1u8; TILEMAP_SIZE * TILEMAP_SIZE] });
        }
    }

    let input_map = input::InputMap::new();
    let mut input_state = input::InputState::new();

    // Allocate cpu side v_buf
    let v_buf = vec![Default::default(); renderer::V_BUF_SIZE];
    world.add_resource(atlas);
    world.add_resource(camera);
    world.add_resource(input_state.clone());
    world.add_resource(DeltaTime(0.016));
    world.add_resource(renderer::VertexBuffer {
        v_buf: v_buf, size: 0,
    });

    // Build dispatcher
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(sys_control::PlayerControllerSys, "player_controller", &[])
        .with(sys_phys::PhysSys::<CollCircle, CollCircle>::new(), "phys_circ_circ", &["player_controller"])
        .with(sys_anim::AnimSpriteSys, "anim_sprite", &["player_controller"])
        .with(MarkerSys, "update", &["phys_circ_circ", "player_controller"])
        .with(renderer::TilemapPainter, "tilemap_paint", &["update"])
        .with(renderer::AnimSpritePainter, "anim_sprite_paint", &["update"])
        .with(renderer::StaticSpritePainter, "static_sprite_paint", &["update"])
        .build();

    let mut should_close = false;

    // Number of frames until we print another frame time
    let mut fps_count_timer = 60;
    while !should_close {
        let start = time::Instant::now();
        input_state.process_input(&input_map, &mut events_loop);
        should_close = input_state.should_close;
        if should_close { break; } // Early return for speedy exit

        // Add input
        world.add_resource(input_state.clone());
        dispatcher.dispatch(&mut world.res);
        let mut v_buf = world.write_resource::<renderer::VertexBuffer>();
        renderer.flush_render(&mut device, &v_buf, &world.read_resource::<renderer::Camera>());
        v_buf.size = 0; // After painting, we need to clear the v_buf

        window.swap_buffers().unwrap();
        device.cleanup();
        let elapsed = start.elapsed();
        if fps_count_timer <= 0 {
            println!("Time taken (millis): {:?}",
                     elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64);
            fps_count_timer = 60;
        }
        fps_count_timer -= 1;
    }
}
