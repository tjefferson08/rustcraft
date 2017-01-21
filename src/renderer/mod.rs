extern crate glium;
extern crate image;
extern crate cgmath;

use glium::{Frame,Program,Surface};
use rectangle::Rectangle;
use cgmath::*;
use entity::Entity;
use std::ops::Neg;
use rectangle::Model;

pub struct Master {
    pub target: Frame
}

impl Master {
    pub fn new(target: Frame) -> Master {
        Master {
            target: target
        }
    }

    pub fn clear(&mut self) -> () {
        self.target.clear_color(0.0, 0.0, 1.0, 1.0);
    }

    pub fn draw(&mut self, rect: &Rectangle, program: &Program) {
        let camera = Entity {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0)
        };

        self.target.draw(
            &rect.positions(),
            &rect.indices(),
            program,
            &uniform! {
                model_matrix: rect.model_matrix(),
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
            aspect: 1024.0 / 768.0,
            near: 0.001,
            far: 1000.0
        }
    );
    mat.into()
}


fn pretty_print4(m: Matrix4<f32>) {
    println!(" {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10},", m.x.x, m.y.x, m.z.x, m.w.x);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10},", m.x.y, m.y.y, m.z.y, m.w.y);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10},", m.x.z, m.y.z, m.z.z, m.w.z);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10} ", m.x.w, m.y.w, m.z.w, m.w.w);
    println!("");
}

fn pretty_print3(m: Matrix3<f32>) {
    println!(" {:>13.10}, {:>13.10}, {:>13.10}", m.x.x, m.y.x, m.z.x);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}", m.x.y, m.y.y, m.z.y);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}", m.x.z, m.y.z, m.z.z);
    println!("");
}

