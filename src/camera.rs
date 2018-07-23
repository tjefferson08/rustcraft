use cgmath::{Deg, Matrix3, Matrix4, PerspectiveFov, Rad, Vector3};
use constants::*;
use entity::Entity;
use std::ops::Neg;
use window::{HEIGHT as WINDOW_HEIGHT, WIDTH as WINDOW_WIDTH};

pub struct Camera {
    pub entity: Entity,
}

impl Camera {
    pub fn new(pos: Vector3<f32>) -> Camera {
        Camera {
            entity: Entity::new(pos),
        }
    }

    pub fn reset(&mut self) -> () {
        self.entity.position = Vector3::new(0.0, 0.0, 0.0);
        self.entity.rotation = Vector3::new(0.0, 0.0, 0.0);
    }

    pub fn update_position(&mut self, pos: Vector3<f32>) -> () {
        self.entity.position += pos;
    }

    pub fn update_rotation(&mut self, rot: Vector3<f32>) -> () {
        let print = false;
        if print && (rot.x > 0.0 || rot.y > 0.0 || rot.z > 0.0) {
            println!(
                "about to rotate camera by {} {} {}",
                rot.x.to_degrees(),
                rot.y.to_degrees(),
                rot.z.to_degrees()
            );
            println!(
                "camera rotation before update {} {} {}",
                self.entity.rotation.x.to_degrees(),
                self.entity.rotation.y.to_degrees(),
                self.entity.rotation.z.to_degrees()
            );
        }
        self.entity.rotation.x = self.entity.rotation.x + rot.x;

        // up/down camera should stop at top/bottom
        if self.entity.rotation.x > DEG_TO_RAD_90 {
            self.entity.rotation.x = DEG_TO_RAD_90
        }
        if self.entity.rotation.x < -DEG_TO_RAD_90 {
            self.entity.rotation.x = -DEG_TO_RAD_90
        }

        // left/right camera should not stop, but let's keep values
        // b/t 0 and 2pi to stay sane
        if self.entity.rotation.y < 0.0 {
            self.entity.rotation.y = DEG_TO_RAD_360;
        }

        if self.entity.rotation.y > DEG_TO_RAD_360 {
            self.entity.rotation.y = 0.0;
        }

        self.entity.rotation.y = self.entity.rotation.y + rot.y;

        // let's not worry about rotating the camera around Z axis
        // self.entity.rotation.z = self.entity.rotation.z + rot.z;

        if print && (rot.x > 0.0 || rot.y > 0.0 || rot.z > 0.0) {
            println!(
                "camera rotation after update {} {} {}",
                self.entity.rotation.x.to_degrees(),
                self.entity.rotation.y.to_degrees(),
                self.entity.rotation.z.to_degrees()
            );
        }
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let pos = self.entity.position;
        let rot = self.entity.rotation;
        let rot3_x: Matrix3<f32> = Matrix3::from_angle_x(Rad(rot.x));
        let rot3_y: Matrix3<f32> = Matrix3::from_angle_y(Rad(rot.y));
        let rot3_z: Matrix3<f32> = Matrix3::from_angle_z(Rad(rot.z));
        let rot_3d = Matrix4::from(rot3_x * rot3_y * rot3_z);

        let view_matrix: Matrix4<f32> =
            rot_3d * Matrix4::from_translation(Vector3::new(pos.x, pos.y, pos.z).neg());

        view_matrix.into()
    }

    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
        let mat = Matrix4::from(PerspectiveFov {
            fovy: Deg(120.0f32).into(),
            aspect: WINDOW_WIDTH / WINDOW_HEIGHT,
            near: 0.001,
            far: 1000.0,
        });
        mat.into()
    }
}
