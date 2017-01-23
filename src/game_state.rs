use std::vec::Vec;
use model::{Model,Rectangle};
use camera::Camera;
use renderer::Master;
use glium::glutin::Event;
use glium::glutin;

pub struct PlayingState {
    models: Vec<Box<Model>>,
    camera: Camera
}

pub trait GameState {
    fn input(&self) -> ();
    fn process_event(&mut self, event: Event) -> ();
    fn update(&mut self) -> ();
    fn draw(&self, renderer: &mut Master) -> ();
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

    fn process_event(&mut self, event: Event) -> () {
        match event {
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Down)
            ) => {
                self.camera.update((0.0, -0.1, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Up)
            ) => {
                self.camera.update((0.0, 0.1, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Left)
            ) => {
                self.camera.update((-0.1, 0.0, 0.0));
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::Right)
            ) => {
                self.camera.update((0.1, 0.0, 0.0));
            },
            _ => ()
        }
    }

    fn update(&mut self) {
        for mut model in self.models.iter_mut() {
            // model.update_position((0.01, 0.0, 0.0));
            model.update_rotation((0.01, 0.01, 0.0));
        }
    }

    fn draw(&self, renderer: &mut Master) {
        for  model in self.models.iter() {
            renderer.draw(
                model.as_ref(),
                &self.camera
            );
        }
    }
}
