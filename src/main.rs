#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate rand;
extern crate failure;
extern crate image;
extern crate rusttype;
#[macro_use] extern crate failure_derive;

mod renderer;

use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlRequest, GlContext};
use glutin::Api::OpenGl;

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
    let mut renderer = renderer::Renderer::new(&mut factory, color_view, w, h, Default::default());

    // Setup vertices and uniforms
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
        renderer.render(&mut device,
                        renderer::RenderState {});
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
