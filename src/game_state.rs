use std::vec::Vec;
use model::{Cube,Model};
use camera::Camera;
use renderer::Master;
use glium::glutin::Event;
use glium::glutin;
use constants::*;
use cgmath::Vector3;
    
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
            camera: Camera::new(Vector3::new(0.0, 2.0, 0.0))
        };

        let cubeN = Cube::from_position(Vector3::new(0.0, 0.0, -5.0));
        let cubeS = Cube::from_position(Vector3::new(0.0, 0.0, 5.0));
        let cubeE = Cube::from_position(Vector3::new(5.0, 0.0, 0.0));
        let cubeW = Cube::from_position(Vector3::new(-5.0, 0.0, 0.0));
        // let rect2: Rectangle = Rectangle::from_coords((1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        playing_state.models.push(
            Box::new(cubeN)
        );
        playing_state.models.push(
            Box::new(cubeS)
        );
        playing_state.models.push(
            Box::new(cubeE)
        );
        playing_state.models.push(
            Box::new(cubeW)
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
        let speed = 50.0;
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
                    self.camera.entity.position.x,
                    self.camera.entity.position.y,
                    self.camera.entity.position.z
                );
                println!(
                    "camera rotation {} {} {}",
                    self.camera.entity.rotation.x,
                    self.camera.entity.rotation.y,
                    self.camera.entity.rotation.z
                );
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::W)
            ) => {
                let delta_pos = Vector3::new(
                    -((self.camera.entity.rotation.y + DEG_TO_RAD_90).cos() * speed * delta_t),
                    0.0,
                    -((self.camera.entity.rotation.y + DEG_TO_RAD_90).sin() * speed * delta_t)
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::S)
            ) => {
                let delta_pos = Vector3::new(
                    (self.camera.entity.rotation.y + DEG_TO_RAD_90).cos() * speed * delta_t,
                    0.0,
                    (self.camera.entity.rotation.y + DEG_TO_RAD_90).sin() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::A)
            ) => {
                let delta_pos = Vector3::new(
                    -self.camera.entity.rotation.y.cos() * speed * delta_t,
                    0.0,
                    -self.camera.entity.rotation.y.sin() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            
            glutin::Event::KeyboardInput(
                glutin::ElementState::Pressed,
                _,
                Some(glutin::VirtualKeyCode::D)
            ) => {
                let delta_pos = Vector3::new(
                    self.camera.entity.rotation.y.cos() * speed * delta_t,
                    0.0,
                    self.camera.entity.rotation.y.sin() * speed * delta_t
                );
                self.camera.update_position(delta_pos);
            },
            _ => ()
        }
    }

    fn process_mouse_move(&mut self, deflection: (i32, i32), delta_t: f32) -> () {
        let rotation_speed = 1.0 * delta_t;
        const MAX_DEFLECTION: f32 = 10.0;
        const MIN_DEFLECTION: f32 = -10.0;
        if deflection.0.abs() > 700 || deflection.1.abs() > 700 {
            println!("GOOD GOD");
        }
        
        let deflection_x = rotation_speed *
            (deflection.1 as f32).min(MAX_DEFLECTION).max(MIN_DEFLECTION);
        
        let deflection_y = rotation_speed *
            (deflection.0 as f32).min(MAX_DEFLECTION).max(MIN_DEFLECTION);
        
        // if deflection.0.abs() > 1 || deflection.1.abs() > 1 {
        //     println!("mouse deflection {} {}", deflection.0, deflection.1);
        //     println!("culled deflection {} {}", deflection_x, deflection_y);
        // }

        self.camera.update_rotation(
            Vector3::new(
                deflection_x,
                deflection_y,
                // 0.0,
                0.0
            )
        );
    }

    fn update(&mut self, delta_t: f32) {
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
