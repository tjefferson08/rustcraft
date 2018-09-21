extern crate glium;
extern crate image;

use game_state::{GameState, PlayingState};
use glium::glutin;
use glium::glutin::{DeviceEvent, Event, EventsLoop, WindowEvent};
use renderer;
use std::time::Instant;
use window::{Window, HEIGHT, WIDTH};

pub struct Application {
    state_stack: Vec<Box<GameState>>,
}

impl Application {
    pub fn new() -> Application {
        let mut app = Application {
            state_stack: Vec::new(),
        };
        let playing_state: PlayingState = PlayingState::new();
        let boxed_state: Box<PlayingState> = Box::new(playing_state);
        app.push_state(boxed_state);
        app
    }

    pub fn run_game_loop(&mut self) -> () {
        let mut events_loop = EventsLoop::new();
        let window = Window::new(&events_loop);

        let mut now = Instant::now();
        let mut current_mouse = (0, 0);
        let mut pressed_keys: [bool; 256] = [false; 256];
        let mut closed = false;
        while !closed {
            // don't care about seconds portion
            let time_since_previous_loop = now.elapsed();
            let delta_t = (time_since_previous_loop.subsec_nanos() as f32) / 1000_000_000.0;
            now = Instant::now();

            // do immutable things with self
            {
                let current_state = self.current_state();
                let mut master_renderer =
                    renderer::Master::new(window.display(), window.display().draw());
                master_renderer.clear();
                current_state.input();
                current_state.draw(&mut master_renderer, delta_t);
                master_renderer.update();
            }

            let mut current_state = self.current_state_mut();
            let mut mouse_deflection = (0, 0);

            // now we can do mutable things
            let events = events_loop.poll_events(|event| {
                // assume current mouse is in 'reset' position (in case
                // there ARE no mouse events this loop, we don't want to
                // keep "pulling" in whatever direction we last deflected)
                current_mouse = (WIDTH as i32, HEIGHT as i32);
                match event {
                    // N.B. glium::glutin::Event::WindowEvent is not
                    // the same as glium::glutin::WindowEvent
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            closed = true;
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    state: glutin::ElementState::Pressed,
                                    virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => {
                            closed = true;
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    state, scancode, ..
                                },
                            ..
                        } => {
                            pressed_keys[scancode as usize] =
                                state == glutin::ElementState::Pressed;
                        }
                        _ => return,
                    },

                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::MouseMotion { delta } => {
                            if mouse_deflection != (0, 0) {
                                println!(
                                    "throwing away old deflection {}, {}",
                                    mouse_deflection.0, mouse_deflection.1
                                );
                            }
                            mouse_deflection = (delta.0 as i32, delta.1 as i32);
                        }
                        _ => return,
                    },
                    _ => (),
                }
            });
            current_state.process_keyboard_input(&pressed_keys, delta_t);
            current_state.process_mouse_move(mouse_deflection, delta_t);
            current_state.update(delta_t);
        }
    }

    pub fn push_state(&mut self, game_state: Box<GameState>) -> () {
        self.state_stack.push(game_state)
    }

    pub fn pop_state(&mut self) -> () {
        self.state_stack.pop();
    }

    pub fn current_state(&self) -> &Box<GameState> {
        self.state_stack.last().unwrap()
    }

    pub fn current_state_mut(&mut self) -> &mut Box<GameState> {
        self.state_stack.last_mut().unwrap()
    }
}
