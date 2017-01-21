extern crate glium;
extern crate image;

use game_state::{GameState, PlayingState};
use window;
use rectangle;
use renderer;
use shaders;
use textures;

pub struct Application {
    state_stack: Vec<Box<GameState>>
}

impl Application {
    pub fn new() -> Application {
        let mut app = Application {
            state_stack: Vec::new()
        };

        app.push_state(
            Box::new(PlayingState::new(1))
        );

        app
    }

    pub fn run_game_loop(&self) -> () {
        let state = self.current_state();
        match state {
            None => println!("state stack is empty"),
            Some(current_state) => {
                println!("current state is real");
                current_state.input();
                current_state.update();
                current_state.draw();
            }
        }

        let window = window::new();

        let rect = rectangle::Rectangle::new(window.display());

        let vertex_shader_src = shaders::load("src/shaders/vertex_shader.glsl");
        let fragment_shader_src = shaders::load("src/shaders/fragment_shader.glsl");
        let texture = textures::load("src/textures/grass.png", window.display());

        let program = glium::Program::from_source(
            window.display(),
            &vertex_shader_src,
            &fragment_shader_src,
            None
        ).unwrap();

        let mut t: f32 = -0.5;

        while window.is_open() {

            // we update `t`
            t += 0.0002;
            if t > 0.5 {
                t = -0.5;
            }

            let mut master_renderer = renderer::Master::new(window.display().draw());
            master_renderer.clear();

            master_renderer.draw(
                &rect,
                &program
            );
            master_renderer.update();
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

    pub fn current_state(&self) -> Option<&Box<GameState>> {
        println!("current state");
        self.state_stack.last()
    }
}


