extern crate glium;
extern crate image;
extern crate cgmath;

use glium::{Frame,Program,Surface};
use model::{Model};
use cgmath::*;
use entity::Entity;
use std::ops::Neg;
use glium::backend::glutin_backend::GlutinFacade;
use window::{HEIGHT as WINDOW_HEIGHT, WIDTH as WINDOW_WIDTH};
use shaders;

pub struct Master<'a>{
    display: &'a GlutinFacade,
    pub target: Frame,
    program: Program
}

impl<'a> Master<'a> {
    pub fn new(display: &GlutinFacade, target: Frame) -> Master {
        let vertex_shader_src = shaders::load("src/shaders/vertex_shader.glsl");
        let fragment_shader_src = shaders::load("src/shaders/fragment_shader.glsl");

        Master {
            display: display,
            target: target,
            program: Program::from_source(
                display,
                &vertex_shader_src,
                &fragment_shader_src,
                None
            ).unwrap()
        }
    }

    pub fn clear(&mut self) -> () {
        self.target.clear_color(0.0, 0.0, 1.0, 1.0);
    }

    pub fn draw(&mut self, model: &Model) {
        let camera = Entity {
            position: (0.0, 0.0, -1.0),
            rotation: (0.0, 0.0, 0.0)
        };

        self.target.draw(
            &model.positions(self.display),
            &model.indices(self.display),
            &self.program,
            &uniform! {
                model_matrix: model.model_matrix(),
                view_matrix: view_matrix(&camera),
                projection_matrix: projection_matrix()
            },
            &Default::default()
        ).unwrap()
    }

    pub fn update(self) -> () {
        self.target.finish().unwrap();
    }


}

pub fn view_matrix(entity: &Entity) -> [[f32; 4]; 4] {
    let (pos_x, pos_y, pos_z) = entity.position;
    let (rot_x, rot_y, rot_z) = entity.rotation;
    let rot3_x: Matrix3<f32> = Matrix3::from_angle_x(
        Rad(rot_x)
    );
    let rot3_y: Matrix3<f32> = Matrix3::from_angle_y(
        Rad(rot_y)
    );
    let rot3_z: Matrix3<f32> = Matrix3::from_angle_z(
        Rad(rot_z)
    );
    let view_matrix: Matrix4<f32> =
        Matrix4::from_translation(
            Vector3::new(pos_x, pos_y, pos_z).neg()
        ) * Matrix4::from(rot3_x * rot3_y * rot3_z);

    view_matrix.into()
}

pub fn projection_matrix() -> [[f32; 4]; 4] {
    let mat = Matrix4::from(
        PerspectiveFov {
            fovy: Deg(120.0f32).into(),
            aspect: WINDOW_WIDTH / WINDOW_HEIGHT,
            near: 0.001,
            far: 1000.0
        }
    );
    mat.into()
}


