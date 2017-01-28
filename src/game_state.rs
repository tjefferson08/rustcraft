use std::vec::Vec;
use model::{Cube,Model,Rectangle};
use camera::Camera;
use renderer::Master;
use glium::glutin::Event;
use glium::glutin;
use std::f32::consts;
use constants::*;

pub struct PlayingState {
    models: Vec<Box<Model>>,
    camera: Camera
}

pub trait GameState {
    fn input(&self) -> ();
    fn process_event(&mut self, event: Event, delta_t: f32) -> ();
    fn process_mouse_move(&mut self, deflection: (i32, i32), delta_t: f32) -> ();
    fn update(&mut self, delta_t: f32) -> ();
    fn draw(&self, renderer: &mut Master, delta_t: f32) -> ();
}

impl PlayingState {
    pub fn new() -> PlayingState {

        let mut playing_state = PlayingState {
            models: Vec::new(),
            camera: Camera::new((0.0, 2.0, 0.0))
        };

        let cube = Cube::new();
        // let rect2: Rectangle = Rectangle::from_coords((1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        playing_state.models.push(
            Box::new(cube)
        );
        // playing_state.models.push(
        //     Box::new(rect2)
        // );

        let immut_ps = playing_state;
        immut_ps
    }
}

impl GameState for PlayingState {
    fn input(&self) {
        // println!("input");
    }

    fn process_event(&mut self, event: Event, delta_t: f32) -> () {
        let speed = 5.0;
        match event {
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::R)
            ) => {
                self.camera.reset()
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::C)
            ) => {
                println!(
                    "camera position {} {} {}",
                    self.camera.entity.position.0,
                    self.camera.entity.position.1,
                    self.camera.entity.position.2
                );
                println!(
                    "camera rotation {} {} {}",
                    self.camera.entity.rotation.0,
                    self.camera.entity.rotation.1,
                    self.camera.entity.rotation.2
                );
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::W)
            ) => {
                let delta_pos = (
                    -((self.camera.entity.rotation.1 + DEG_TO_RAD_90).cos() * speed * delta_t),
                    0.0,
                    -((self.camera.entity.rotation.1 + DEG_TO_RAD_90).sin() * speed * delta_t)
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::S)
            ) => {
                let delta_pos = (
                    (self.camera.entity.rotation.1 + DEG_TO_RAD_90).cos() * speed * delta_t,
                    0.0,
                    (self.camera.entity.rotation.1 + DEG_TO_RAD_90).sin() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::A)
            ) => {
                let delta_pos = (
                    -self.camera.entity.rotation.1.cos() * speed * delta_t,
                    0.0,
                    -self.camera.entity.rotation.1.sin() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::D)
            ) => {
                let delta_pos = (
                    self.camera.entity.rotation.1.cos() * speed * delta_t,
                    0.0,
                    self.camera.entity.rotation.1.sin() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            _ => ()
        }
    }

    fn process_mouse_move(&mut self, deflection: (i32, i32), delta_t: f32) -> () {
        let rotation_speed = 1.0 * delta_t;
        let mut MAX_DEFLECTION = 10.0;
        let mut MIN_DEFLECTION = -10.0;
        if deflection.0.abs() > 700 || deflection.1.abs() > 700 {
            println!("GOOD GOD");
        }
        
        let mut deflection_x = (rotation_speed *
                                (deflection.1 as f32)
                                .min(MAX_DEFLECTION)
                                .max(MIN_DEFLECTION));
                                
        let mut deflection_y = (
            rotation_speed * (deflection.0 as f32).min(MAX_DEFLECTION).max(MIN_DEFLECTION)
        );
        
        // if deflection.0.abs() > 1 || deflection.1.abs() > 1 {
        //     println!("mouse deflection {} {}", deflection.0, deflection.1);
        //     println!("culled deflection {} {}", deflection_x, deflection_y);
        // }

        self.camera.update_rotation(
            (
                deflection_x,
                deflection_y,
                // 0.0,
                0.0
            )
        );
    }

    fn update(&mut self, delta_t: f32) {
        self.camera.update_rotation((0.0, 0.0, 0.01));

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
