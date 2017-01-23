extern crate glium;
extern crate image;

use game_state::{GameState, PlayingState};
use window::Window;
use camera::Camera;
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

        loop {
            t += 0.0002;
            if t > 0.5 {
                t = -0.5;
            }

            // do immutable things with self
            {
                let current_state = self.current_state();
                let mut master_renderer = renderer::Master::new(window.display(), window.display().draw());
                master_renderer.clear();
                current_state.input();
                current_state.draw(&mut master_renderer);
                master_renderer.update();
            }

            // now we can do mutable things
            let events = window.display().poll_events();
            let mut st = self.current_state_mut();
            for event in events {
                match event {
                    glium::glutin::Event::Closed => return,
                    ev => st.process_event(ev)
                }
            }
            st.update();
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


