extern crate glium;

use game_state::{GameState, PlayingState};
use glium::{DisplayBuild, Surface};
use window;
use teapot;

pub struct Application {
    state_stack: Vec<Box<GameState>>,
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

        let positions = glium::VertexBuffer::new(&window.display, &teapot::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(&window.display, &teapot::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(
            &window.display,
            glium::index::PrimitiveType::TrianglesList,
            &teapot::INDICES
        ).unwrap();

        let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

        let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;

        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

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
}


