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
#[macro_use] extern crate specs_derive;

mod renderer;
mod comp;
mod input;
mod sys_control;
mod sys_phys;

use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlRequest, GlContext};
use glutin::Api::OpenGl;

/// Create the world and register all the components
fn create_world() -> specs::World {
    let mut world = specs::World::new();
    world.register::<comp::Pos>();
    world.register::<comp::Vel>();
    world.register::<comp::PlayerControlled>();
    world.register::<comp::DebugRender>();
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
    let (window, mut device, mut factory, color_view, _depth_view) =
        gfx_glutin::init::<renderer::ColorFormat, renderer::DepthFormat>(
            windowbuilder, contextbuilder, &events_loop);


    let (w, h) = window.get_inner_size().unwrap();
    let (mut renderer, atlas) = renderer::Renderer::new(&mut factory, color_view, w, h, Default::default());

    // Create the ECS world, and a test entity
    let mut world = create_world();
    use specs::Builder;
    world.create_entity()
        .with(comp::Pos { x: 100.0, y: 100.0 })
        .with(comp::Vel { x: 0.0, y: 0.0 })
        .with(comp::PlayerControlled { move_speed: 100.0 })
        .with(comp::DebugRender { w: 64.0, h: 64.0, col: [1.0, 0.0, 0.0, 1.0] });

    let input_map = input::InputMap::new();
    let mut input_state = input::InputState::new();

    // Allocate cpu side v_buf
    let v_buf = vec![Default::default(); renderer::V_BUF_SIZE];
    world.add_resource(atlas);
    world.add_resource(input_state.clone());
    world.add_resource(sys_phys::DeltaTime(0.016));
    world.add_resource(renderer::VertexBuffer {
        v_buf: v_buf, size: renderer::V_BUF_SIZE as u32,
    });

    // Build dispatcher
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(sys_control::PlayerControllerSys, "player_controller", &[])
        .with(sys_phys::PhysSys, "phys", &["player_controller"])
        .with_thread_local(renderer::Painter)
        .build();

    let mut should_close = false;

    while !should_close {
        input_state.process_input(&input_map, &mut events_loop);
        should_close = input_state.should_close;
        if should_close { break; } // Early return for speedy exit

        // Add input
        world.add_resource(input_state.clone());
        dispatcher.dispatch(&mut world.res);
        let v_buf = world.read_resource::<renderer::VertexBuffer>();
        renderer.flush_render(&mut device, &v_buf);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
