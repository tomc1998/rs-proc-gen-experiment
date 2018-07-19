//! Module for handling input maps & reading input from the event loop

use std::collections::HashMap;
use glutin;
use fpa::*;
use fpavec::*;

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
    /// Used to attack, but also to navigate through dialogues
    Primary,
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
        map.insert(Input::Mouse(glutin::MouseButton::Left), Command::Primary);
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
    /// Has <command> just been pressed?
    pub pressed: HashMap<Command, bool>,
    /// Mouse in world coordinates
    pub mouse: Vec32,
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
        down.insert(Command::Primary, false);
        let mut pressed = HashMap::new();
        pressed.insert(Command::MoveLeft, false);
        pressed.insert(Command::MoveRight, false);
        pressed.insert(Command::MoveDown, false);
        pressed.insert(Command::MoveUp, false);
        pressed.insert(Command::Primary, false);
        InputState {
            down: down,
            pressed: pressed,
            should_close: false,
            window_size: (0, 0),
            mouse: Vec32::zero(),
        }
    }
}

impl InputState {
    pub fn new() -> InputState {
        Default::default()
    }

    pub fn process_input(&mut self, map: &InputMap, events_loop: &mut glutin::EventsLoop) {
        for (_, v) in self.pressed.iter_mut() {
            *v = false;
        }
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
                            if e == glutin::ElementState::Pressed {
                                if !self.down.get(&c).unwrap() {
                                    self.pressed.insert(c.clone(), true);
                                }
                                self.down.insert(c.clone(), true);
                            } else {
                                self.down.insert(c.clone(), false);
                            }
                        }
                        _ => ()
                    }
                    // Check for mouse commands
                    glutin::WindowEvent::MouseInput {
                        state: e,
                        button: b, ..
                    } => match map.get(&Input::Mouse(b)) {
                        Some(c) => {
                            if e == glutin::ElementState::Pressed {
                                if !self.down.get(&c).unwrap() {
                                    self.pressed.insert(c.clone(), true);
                                }
                                self.down.insert(c.clone(), true);
                            } else {
                                self.down.insert(c.clone(), false);
                            }
                        }
                        _ => ()
                    }
                    // Update mouse pos
                    glutin::WindowEvent::CursorMoved {
                        position: (x, y), ..
                    } => {
                        // TODO: Scale to world pos with camera
                        self.mouse.x = Fx32::new(x as f32);
                        self.mouse.y = Fx32::new(y as f32);
                    }
                    _ => {},
                }
            }
        });
    }
}
