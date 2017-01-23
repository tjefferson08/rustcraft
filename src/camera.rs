use entity::Entity;
use cgmath::{
    Deg,
    Matrix3,
    Matrix4,
    PerspectiveFov,
    Rad,
    Vector3
};
use std::ops::Neg;
use window::{HEIGHT as WINDOW_HEIGHT, WIDTH as WINDOW_WIDTH};

pub struct Camera {
    entity: Entity
}

impl Camera {
    pub fn new(pos: (f32, f32, f32)) -> Camera {
        Camera {
            entity: Entity {
                position: pos,
                rotation: (0.0, 0.0, 0.0)
            }
        }
    }

    pub fn update(

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let (pos_x, pos_y, pos_z) = self.entity.position;
        let (rot_x, rot_y, rot_z) = self.entity.rotation;
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

    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
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
}
