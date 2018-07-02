//! Module for handling input maps & reading input from the event loop

use std::collections::HashMap;
use glutin;

/// Some input from the player, used for mapping inputs to commands
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Input {
    Key(glutin::VirtualKeyCode),
    Mouse(glutin::MouseButton),
}

/// A command to be issued as a result of some input
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
}

/// A mapping of inputs to commands
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InputMap {
    map: HashMap<Input, Command>,
}

impl InputMap {
    pub fn new() -> InputMap {
        let mut map = HashMap::new();
        map.insert(Input::Key(glutin::VirtualKeyCode::W), Command::MoveUp);
        map.insert(Input::Key(glutin::VirtualKeyCode::A), Command::MoveLeft);
        map.insert(Input::Key(glutin::VirtualKeyCode::S), Command::MoveDown);
        map.insert(Input::Key(glutin::VirtualKeyCode::D), Command::MoveRight);
        InputMap {
            map: map,
        }
    }

    /// Add a mapping
    #[allow(dead_code)]
    pub fn add(&mut self, i: Input, c: Command) -> &mut Self {
        self.map.insert(i, c);
        self
    }

    pub fn get(&self, i: &Input) -> Option<&Command> {
        self.map.get(&i)
    }
}

/// The current state of input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    /// Is <command> down?
    pub down: HashMap<Command, bool>,
    /// Was a close requested?
    pub should_close: bool,
    /// The size of the window
    pub window_size: (u32, u32),
}

impl Default for InputState {
    fn default() -> Self {
        let mut down = HashMap::new();
        down.insert(Command::MoveLeft, false);
        down.insert(Command::MoveRight, false);
        down.insert(Command::MoveDown, false);
        down.insert(Command::MoveUp, false);
        InputState {
            down: down,
            should_close: false,
            window_size: (0, 0),
        }
    }
}

impl InputState {
    pub fn new() -> InputState {
        Default::default()
    }

    pub fn process_input(&mut self, map: &InputMap, events_loop: &mut glutin::EventsLoop) {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Resized(w, h) =>
                        self.window_size = (w, h),
                    glutin::WindowEvent::CloseRequested |
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => self.should_close = true,
                    // Check for keyboard commands
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            state: e,
                            virtual_keycode: Some(k), ..
                        }, ..
                    } => match map.get(&Input::Key(k)) {
                        Some(c) => {
                            self.down.insert(c.clone(), match e {
                                glutin::ElementState::Pressed => true,
                                glutin::ElementState::Released => false,
                            }).unwrap();
                        }
                        _ => ()
                    }
                    // Check for mouse commands
                    glutin::WindowEvent::MouseInput {
                        state: e,
                        button: b, ..
                    } => match map.get(&Input::Mouse(b)) {
                        Some(c) => {
                            self.down.insert(c.clone(), match e {
                                glutin::ElementState::Pressed => true,
                                glutin::ElementState::Released => false,
                            }).unwrap();
                        }
                        _ => ()
                    }
                    _ => {},
                }
            }
        });
    }
}
