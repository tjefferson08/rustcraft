use camera::Camera;
use cgmath::Vector3;
use constants::*;
use glium::glutin::Event;
use model::{Cube, Model};
use renderer::Master;
use std::vec::Vec;

pub struct PlayingState {
    models: Vec<Box<Model>>,
    camera: Camera,
}

pub trait GameState {
    fn input(&self) -> ();
    fn process_event(&mut self, event: Event, delta_t: f32) -> ();
    fn process_keyboard_input(&mut self, pressed_keys: &[bool; 256], delta_t: f32) -> ();
    fn process_mouse_move(&mut self, deflection: (i32, i32), delta_t: f32) -> ();
    fn update(&mut self, delta_t: f32) -> ();
    fn draw(&self, renderer: &mut Master, delta_t: f32) -> ();
}

impl PlayingState {
    pub fn new() -> PlayingState {
        let mut playing_state = PlayingState {
            models: Vec::new(),
            camera: Camera::new(Vector3::new(0.0, 2.0, 0.0)),
        };

        let cube_n = Cube::from_position(Vector3::new(0.0, 0.0, -5.0));
        let cube_s = Cube::from_position(Vector3::new(0.0, 0.0, 5.0));
        let cube_e = Cube::from_position(Vector3::new(5.0, 0.0, 0.0));
        let cube_w = Cube::from_position(Vector3::new(-5.0, 0.0, 0.0));
        // let rect2: Rectangle = Rectangle::from_coords((1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        playing_state.models.push(Box::new(cube_n));
        playing_state.models.push(Box::new(cube_s));
        playing_state.models.push(Box::new(cube_e));
        playing_state.models.push(Box::new(cube_w));
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
            _ => (),
        }
    }

    fn process_keyboard_input(&mut self, pressed_keys: &[bool; 256], delta_t: f32) -> () {
        let speed = 10.0;
        let mut delta_pos = Vector3::new(0.0, 0.0, 0.0);

        // R
        if pressed_keys[15] {
            self.camera.reset();
        }

        // C
        if pressed_keys[8] {
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
        }

        // W
        if pressed_keys[13] {
            delta_pos += Vector3::new(
                -((self.camera.entity.rotation.y + DEG_TO_RAD_90).cos() * speed * delta_t),
                0.0,
                -((self.camera.entity.rotation.y + DEG_TO_RAD_90).sin() * speed * delta_t),
            );
        }

        // S
        if pressed_keys[1] {
            delta_pos += Vector3::new(
                (self.camera.entity.rotation.y + DEG_TO_RAD_90).cos() * speed * delta_t,
                0.0,
                (self.camera.entity.rotation.y + DEG_TO_RAD_90).sin() * speed * delta_t,
            );
        }

        // A
        if pressed_keys[0] {
            delta_pos += Vector3::new(
                -self.camera.entity.rotation.y.cos() * speed * delta_t,
                0.0,
                -self.camera.entity.rotation.y.sin() * speed * delta_t,
            );
        }

        // D
        if pressed_keys[2] {
            delta_pos += Vector3::new(
                self.camera.entity.rotation.y.cos() * speed * delta_t,
                0.0,
                self.camera.entity.rotation.y.sin() * speed * delta_t,
            );
        }

        self.camera.update_position(delta_pos);
    }

    fn process_mouse_move(&mut self, deflection: (i32, i32), delta_t: f32) -> () {
        let rotation_speed = 1.0 * delta_t;
        const MAX_DEFLECTION: f32 = 10.0;
        const MIN_DEFLECTION: f32 = -10.0;
        if deflection.0.abs() > 200 || deflection.1.abs() > 200 {
            println!("GOOD GOD {} {}", deflection.0, deflection.1);
        }

        let deflection_x = rotation_speed
            * (deflection.1 as f32)
                .min(MAX_DEFLECTION)
                .max(MIN_DEFLECTION);

        let deflection_y = rotation_speed
            * (deflection.0 as f32)
                .min(MAX_DEFLECTION)
                .max(MIN_DEFLECTION);

        self.camera.update_rotation(Vector3::new(
            deflection_x,
            deflection_y,
            0.0,
        ));
    }

    fn update(&mut self, delta_t: f32) {
        for mut model in self.models.iter_mut() {
            // model.update_position((0.01, 0.0, 0.0));
            model.update_rotation(Vector3::new(0.01, 0.01, 0.0));
        }
    }

    fn draw(&self, renderer: &mut Master, delta_t: f32) {
        for model in self.models.iter() {
            renderer.draw(model.as_ref(), &self.camera);
        }
    }
}
