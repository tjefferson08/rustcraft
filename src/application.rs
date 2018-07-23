extern crate glium;
extern crate image;

use game_state::{GameState, PlayingState};
use glium::glutin;
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
        let window = Window::new();

        let mut now = Instant::now();
        let mut last_mouse = (0, 0);
        let mut current_mouse = (0, 0);
        let mut pressed_keys: [bool; 256] = [false; 256];
        loop {
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

            // now we can do mutable things
            let events = window.display().poll_events();
            let mut current_state = self.current_state_mut();

            window.reset_cursor_position();

            // assume current mouse is in 'reset' position (in case
            // there ARE no mouse events this loop, we don't want to
            // keep "pulling" in whatever direction we last deflected)
            current_mouse = (WIDTH as i32, HEIGHT as i32);
            for event in events {
                match event {
                    glium::glutin::Event::Closed => return,
                    glutin::Event::KeyboardInput(
                        glutin::ElementState::Pressed,
                        _,
                        Some(glutin::VirtualKeyCode::Escape),
                    ) => return,
                    glutin::Event::MouseMoved(x, y) => {
                        current_mouse = (x, y);
                    }
                    glutin::Event::KeyboardInput(element_state, scan_code, _) => {
                        pressed_keys[scan_code as usize] =
                            element_state == glutin::ElementState::Pressed;
                    }
                    _ => (),
                }
            }
            current_state.process_keyboard_input(&pressed_keys, delta_t);

            // measure deflection from center of screen (not sure why
            // this is width/height and not 0.5 * width/height)
            current_state.process_mouse_move(
                (
                    current_mouse.0 - WIDTH as i32,
                    current_mouse.1 - HEIGHT as i32,
                ),
                delta_t,
            );
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
