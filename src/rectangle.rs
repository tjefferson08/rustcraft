extern crate glium;
extern crate cgmath;

use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;
use glium::IndexBuffer;
use cgmath::{
    Matrix3,
    Matrix4,
    Rad,
    Vector3
};

pub struct Rectangle<'a> {
    display: &'a GlutinFacade,

    // (x, y, z)
    position: (f32, f32, f32),
    rotation: (f32, f32, f32)
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: (f32, f32)
}

implement_vertex!(Vertex, position, tex_coords);

const VERTICES: [Vertex; 5] = [
    Vertex { position: (0.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // dummy vector because model indices start at 1

    Vertex { position: (-0.5, -0.5, 0.0), tex_coords: (0.0, 0.0) }, // lower left
    Vertex { position: ( 0.5, -0.5, 0.0), tex_coords: (0.0, 1.0) }, // lower right
    Vertex { position: ( 0.5,  0.5, 0.0), tex_coords: (1.0, 1.0) }, // upper right
    Vertex { position: (-0.5,  0.5, 0.0), tex_coords: (1.0, 0.0) }, // upper left
];

const INDICES: [u16; 6] = [
    1, 3, 4,
    1, 2, 3
];

impl<'a> Rectangle<'a> {
    pub fn new(display: &'a GlutinFacade) -> Rectangle<'a> {
        Rectangle {
            display: display,
            position: (0.0, 0.0, -2.0),
            rotation: (0.0, 0.0, 0.0)
        }
    }
}

pub trait Model {
    fn positions(&self) -> VertexBuffer<Vertex>;
    fn indices(&self) -> IndexBuffer<u16>;
    fn model_matrix(&self) -> [[f32; 4]; 4];
}

impl <'a>Model for Rectangle<'a> {
    fn positions(&self) -> VertexBuffer<Vertex> {
        VertexBuffer::new(self.display, &VERTICES).unwrap()
    }

    fn indices(&self) -> IndexBuffer<u16> {
        IndexBuffer::new(
            self.display,
            glium::index::PrimitiveType::TrianglesList,
            &INDICES
        ).unwrap()
    }

    fn model_matrix(&self) -> [[f32; 4]; 4] {
        let (pos_x, pos_y, pos_z) = self.position;
        let (rot_x, rot_y, rot_z) = self.rotation;
        let rot3_x: Matrix3<f32> = Matrix3::from_angle_x(
            Rad(rot_x)
        );
        let rot3_y: Matrix3<f32> = Matrix3::from_angle_y(
            Rad(rot_y)
        );
        let rot3_z: Matrix3<f32> = Matrix3::from_angle_z(
            Rad(rot_z)
        );
        let model_matrix: Matrix4<f32> =
            Matrix4::from_translation(
                Vector3::new(pos_x, pos_y, pos_z)
            ) * Matrix4::from(rot3_x * rot3_y * rot3_z);

        model_matrix.into()
    }
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

