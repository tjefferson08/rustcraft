extern crate glium;
extern crate image;

use game_state::{GameState, PlayingState};
use window::Window;
use std::time::Instant;
use glium::glutin;
use renderer;
use textures;

pub struct Application {
    state_stack: Vec<Box<GameState>>
}

impl Application {
    pub fn new() -> Application {
        let mut app = Application {
            state_stack: Vec::new()
        };
        let playing_state: PlayingState = PlayingState::new();
        let boxed_state: Box<PlayingState> = Box::new(playing_state);
        app.push_state(boxed_state);
        app
    }

    pub fn run_game_loop(&mut self) -> () {
        let window = Window::new();

        let texture = textures::load("src/textures/grass.png", window.display());

        let mut t: f32 = -0.5;

        let mut now = Instant::now();
        let mut last_mouse = (0, 0);
        let mut current_mouse = (0, 0);

        loop {

            // don't care about seconds portion
            let time_since_previous_loop = now.elapsed();
            let delta_t = (time_since_previous_loop.subsec_nanos() as f32) / 1000_000_000.0;
            now = Instant::now();

            // do immutable things with self
            {
                let current_state = self.current_state();
                let mut master_renderer = renderer::Master::new(window.display(), window.display().draw());
                master_renderer.clear();
                current_state.input();
                current_state.draw(&mut master_renderer, delta_t);
                master_renderer.update();
            }

            // now we can do mutable things
            let events = window.display().poll_events();
            let mut current_state = self.current_state_mut();
            for event in events {
                match event {
                    glium::glutin::Event::Closed => return,
                    glutin::Event::KeyboardInput(
                        glutin::ElementState::Pressed,
                        _,
                        Some(glutin::VirtualKeyCode::Escape)
                    ) => return,
                    glutin::Event::MouseMoved(x, y) => {
                        current_mouse = (x, y);
                    },
                    ev => current_state.process_event(ev, delta_t),
                }
            }
            current_state.process_mouse_move(
                (
                    current_mouse.0 - last_mouse.0,
                    current_mouse.1 - last_mouse.1
                ),
                delta_t
            );
            last_mouse = current_mouse;
            current_state.update(delta_t);
            // window.reset_cursor_position();
        }
    }

    pub fn push_state(&mut self, game_state: Box<GameState>) -> () {
        println!("push state");
        self.state_stack.push(game_state)
    }

    pub fn pop_state(&mut self) -> () {
        println!("pop state");
        self.state_stack.pop();
    }

    pub fn current_state(&self) -> &Box<GameState> {
        self.state_stack.last().unwrap()
    }

    pub fn current_state_mut(&mut self) -> &mut Box<GameState> {
        self.state_stack.last_mut().unwrap()
    }
}


