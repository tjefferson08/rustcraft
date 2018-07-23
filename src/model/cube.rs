extern crate cgmath;
extern crate glium;

use entity::Entity;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Display;
use glium::{IndexBuffer, VertexBuffer};
use glium::{IndexBuffer, VertexBuffer};
use model::{Model, Vertex};
use model::{Model, Vertex};
use textures::{Atlas, TextureCoordinates};

// pub struct Cube<'a> {
pub struct Cube {
    // display: &'a GlutinFacade,

    // (x, y, z)
    entity: Entity,
    vertices: [Vertex; 25],
}

const INDICES: [u16; 36] = [
    // rear
    1,
    2,
    3,
    2,
    4,
    3,
    // front
    5,
    6,
    7,
    6,
    8,
    7,
    // right
    9,
    10,
    11,
    10,
    12,
    11,
    // left
    13,
    14,
    15,
    14,
    16,
    15,
    // bottom
    17,
    18,
    19,
    18,
    20,
    19,
    // top
    21,
    22,
    23,
    22,
    24,
    23,
];

// impl<'a> Cube<'a> {
impl Cube {
    // pub fn new(display: &'a GlutinFacade) -> Cube<'a> {

    pub fn from_position(pos: Vector3<f32>) -> Cube {
        let atlas: Atlas = Atlas::new(16, 512);
        let tex_coords_rear: TextureCoordinates = atlas.texture_coords_for(0, 1);
        let tex_coords_front: TextureCoordinates = atlas.texture_coords_for(0, 1);
        let tex_coords_right: TextureCoordinates = atlas.texture_coords_for(0, 1);
        let tex_coords_left: TextureCoordinates = atlas.texture_coords_for(0, 1);
        let tex_coords_top: TextureCoordinates = atlas.texture_coords_for(0, 0);
        let tex_coords_bottom: TextureCoordinates = atlas.texture_coords_for(0, 2);

        Cube {
            entity: Entity::new(pos),
            vertices: [
                Vertex {
                    position: (0.0, 0.0, 0.0),
                    tex_coords: (0.0, 0.0),
                }, // dummy vertexbecause model indices start at 1
                // rear face
                Vertex {
                    position: (0.0, 1.0, 0.0),
                    tex_coords: tex_coords_rear[0],
                }, // rear upper left
                Vertex {
                    position: (0.0, 0.0, 0.0),
                    tex_coords: tex_coords_rear[1],
                }, // rear lower left
                Vertex {
                    position: (1.0, 1.0, 0.0),
                    tex_coords: tex_coords_rear[2],
                }, // rear upper right
                Vertex {
                    position: (1.0, 0.0, 0.0),
                    tex_coords: tex_coords_rear[3],
                }, // rear lower right
                // front face
                Vertex {
                    position: (0.0, 1.0, 1.0),
                    tex_coords: tex_coords_front[0],
                }, // front upper left
                Vertex {
                    position: (0.0, 0.0, 1.0),
                    tex_coords: tex_coords_front[1],
                }, // front lower left
                Vertex {
                    position: (1.0, 1.0, 1.0),
                    tex_coords: tex_coords_front[2],
                }, // front upper right
                Vertex {
                    position: (1.0, 0.0, 1.0),
                    tex_coords: tex_coords_front[3],
                }, // front lower right
                // right face
                Vertex {
                    position: (1.0, 1.0, 1.0),
                    tex_coords: (0.0, 1.0),
                }, // front upper right
                Vertex {
                    position: (1.0, 0.0, 1.0),
                    tex_coords: (0.0, 0.0),
                }, // front lower right
                Vertex {
                    position: (1.0, 1.0, 0.0),
                    tex_coords: (1.0, 1.0),
                }, // rear upper right
                Vertex {
                    position: (1.0, 0.0, 0.0),
                    tex_coords: (1.0, 0.0),
                }, // rear lower right
                // left face
                Vertex {
                    position: (0.0, 1.0, 0.0),
                    tex_coords: (0.0, 1.0),
                }, // rear upper left
                Vertex {
                    position: (0.0, 0.0, 0.0),
                    tex_coords: (0.0, 0.0),
                }, // rear lower left
                Vertex {
                    position: (0.0, 1.0, 1.0),
                    tex_coords: (1.0, 1.0),
                }, // front upper left
                Vertex {
                    position: (0.0, 0.0, 1.0),
                    tex_coords: (1.0, 0.0),
                }, // front lower left
                // bottom face
                Vertex {
                    position: (0.0, 0.0, 1.0),
                    tex_coords: (0.0, 1.0),
                }, // front lower left
                Vertex {
                    position: (0.0, 0.0, 0.0),
                    tex_coords: (0.0, 0.0),
                }, // rear lower left
                Vertex {
                    position: (1.0, 0.0, 1.0),
                    tex_coords: (1.0, 1.0),
                }, // front lower right
                Vertex {
                    position: (1.0, 0.0, 0.0),
                    tex_coords: (1.0, 0.0),
                }, // rear lower right
                // top face
                Vertex {
                    position: (0.0, 1.0, 0.0),
                    tex_coords: (0.0, 1.0),
                }, // rear upper left
                Vertex {
                    position: (0.0, 1.0, 1.0),
                    tex_coords: (0.0, 0.0),
                }, // front upper left
                Vertex {
                    position: (1.0, 1.0, 0.0),
                    tex_coords: (1.0, 1.0),
                }, // rear upper right
                Vertex {
                    position: (1.0, 1.0, 1.0),
                    tex_coords: (1.0, 0.0),
                }, // front upper right
            ],
        }
    }
}

// impl <'a>Model for Cube<'a> {
impl Model for Cube {
    fn update_position(&mut self, delta_pos: Vector3<f32>) -> () {
        self.entity.position += delta_pos;
    }

    fn update_rotation(&mut self, delta_rot: Vector3<f32>) -> () {
        self.entity.rotation += delta_rot;
    }

    fn positions(&self, disp: &Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(disp, &self.vertices).unwrap()
    }

    fn indices(&self, disp: &Display) -> IndexBuffer<u16> {
        IndexBuffer::new(disp, glium::index::PrimitiveType::TrianglesList, &INDICES).unwrap()
    }

    fn model_matrix(&self) -> [[f32; 4]; 4] {
        let pos = self.entity.position;
        let rot = self.entity.rotation;
        let rot3_x: Matrix3<f32> = Matrix3::from_angle_x(Rad(rot.x));
        let rot3_y: Matrix3<f32> = Matrix3::from_angle_y(Rad(rot.y));
        let rot3_z: Matrix3<f32> = Matrix3::from_angle_z(Rad(rot.z));
        let model_matrix: Matrix4<f32> =
            Matrix4::from_translation(pos) * Matrix4::from(rot3_x * rot3_y * rot3_z);

        model_matrix.into()
    }
}
