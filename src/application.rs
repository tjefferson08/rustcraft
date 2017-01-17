extern crate glium;
extern crate image;

use game_state::{GameState, PlayingState};
use glium::Surface;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::error::Error;
use window;
use rectangle;
use renderer;
use std::io::Cursor;


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

        let image = image::load(
            Cursor::new(&include_bytes!("./textures/grass.png")[..]),
            image::PNG
        ).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            image.into_raw(),
            image_dimensions
        );
        let texture = glium::texture::Texture2d::new(&window.display, image).unwrap();

        // let normals = rectangle::normals(&window.display);
        let indices = rectangle::indices(&window.display);

        let vertex_shader_src = &Application::get_shader("src/shaders/vertex_shader.glsl".to_string());
        let fragment_shader_src = &Application::get_shader("src/shaders/fragment_shader.glsl".to_string());

        let program = glium::Program::from_source(
            &window.display,
            vertex_shader_src,
            fragment_shader_src,
            None
        ).unwrap();

        let mut t: f32 = -0.5;
        let light = [-1.0, 0.4, 0.9f32];

        while window.is_open() {

            // we update `t`
            t += 0.0002;
            if t > 0.5 {
                t = -0.5;
            }

            let matrix = [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ];

            let mut target = window.display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);

            target.draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { matrix: matrix, u_light: light },
                &Default::default()
            ).unwrap();

            target.finish().unwrap();
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

    fn get_shader(filename: String) -> String {
        println!("filename {}", filename);
        let path = Path::new(&filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
           Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => return s
        }
    }
}


