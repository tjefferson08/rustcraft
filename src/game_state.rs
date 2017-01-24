use std::vec::Vec;
use model::{Model,Rectangle};
use camera::Camera;
use renderer::Master;
use glium::glutin::Event;
use glium::glutin;
use std::f32::consts;

pub struct PlayingState {
    models: Vec<Box<Model>>,
    camera: Camera
}

pub trait GameState {
    fn input(&self) -> ();
    fn process_event(&mut self, event: Event, delta_t: f32) -> ();
    fn process_mouse_move(&mut self, deflection: (i32, i32)) -> ();
    fn update(&mut self) -> ();
    fn draw(&self, renderer: &mut Master, delta_t: f32) -> ();
}

impl PlayingState {
    pub fn new() -> PlayingState {

        let mut playing_state = PlayingState {
            models: Vec::new(),
            camera: Camera::new((0.0, 0.0, 0.0))
        };

        let rect1: Rectangle = Rectangle::new();
        let rect2: Rectangle = Rectangle::from_coords((1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        playing_state.models.push(
            Box::new(rect1)
        );
        playing_state.models.push(
            Box::new(rect2)
        );

        let immut_ps = playing_state;
        immut_ps
    }
}

impl GameState for PlayingState {
    fn input(&self) {
        // println!("input");
    }

    fn process_event(&mut self, event: Event, delta_t: f32) -> () {
        let speed = 0.05;
        println!("speed, deltat: {} {}", speed, delta_t);
        match event {
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Down)
            ) => {
                self.camera.update_position((0.0, -0.1, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Up)
            ) => {
                self.camera.update_position((0.0, 0.1, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Left)
            ) => {
                self.camera.update_position((-0.1, 0.0, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Right)
            ) => {
                self.camera.update_position((0.1, 0.0, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::W)
            ) => {
                let delta_pos = (
                    -&self.camera.entity.rotation.1.sin() * speed * delta_t,
                    0.0,
                    -&self.camera.entity.rotation.1.cos() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::A)
            ) => {
                let delta_pos = (
                    -(&self.camera.entity.rotation.1 + (90.0f32).to_radians()).sin() * speed * delta_t,
                    0.0,
                    -(&self.camera.entity.rotation.1 + (90.0f32).to_radians()).cos() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::S)
            ) => {
                let delta_pos = (
                    &self.camera.entity.rotation.1.sin() * speed * delta_t,
                    0.0,
                    &self.camera.entity.rotation.1.cos() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::D)
            ) => {
                let delta_pos = (
                    (&self.camera.entity.rotation.1 + (90.0f32).to_radians()).sin() * speed * delta_t,
                    0.0,
                    (&self.camera.entity.rotation.1 + (90.0f32)).cos() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            _ => ()
        }
    }

    fn process_mouse_move(&mut self, deflection: (i32, i32)) -> () {
        self.camera.update_rotation(
            (0.01 * (deflection.1 as f32), 0.01 * (deflection.0 as f32), 0.0)
        );
    }

    fn update(&mut self) {
        for mut model in self.models.iter_mut() {
            // model.update_position((0.01, 0.0, 0.0));
            // model.update_rotation((0.01, 0.01, 0.0));
        }
    }

    fn draw(&self, renderer: &mut Master, delta_t: f32) {
        for  model in self.models.iter() {
            renderer.draw(
                model.as_ref(),
                &self.camera
            );
        }
    }
}
