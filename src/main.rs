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

use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlRequest, GlContext};
use glutin::Api::OpenGl;

/// Create the world and register all the components
fn create_world() -> specs::World {
    let mut world = specs::World::new();
    world.register::<comp::Pos>();
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
        .with(comp::DebugRender { w: 64.0, h: 64.0, col: [1.0, 0.0, 0.0, 1.0] });


    // Allocate cpu side v_buf
    let v_buf = vec![Default::default(); renderer::V_BUF_SIZE];
    world.add_resource(atlas);
    world.add_resource(renderer::VertexBuffer {
        v_buf: v_buf, size: renderer::V_BUF_SIZE as u32,
    });

    // Build dispatcher
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(renderer::Painter)
        .build();

    let mut should_close = false;

    while !should_close {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Resized(w, h) =>
                        renderer.window_size = (w, h),
                    glutin::WindowEvent::CloseRequested |
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => should_close = true,
                    _ => {},
                }
            }
        });

        dispatcher.dispatch(&mut world.res);
        let v_buf = world.read_resource::<renderer::VertexBuffer>();
        renderer.flush_render(&mut device, &v_buf);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
