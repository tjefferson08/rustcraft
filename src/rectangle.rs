extern crate glium;

use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;
use glium::IndexBuffer;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32),
    tex_coords: (f32, f32)
}

implement_vertex!(Vertex, position, tex_coords);

const VERTICES: [Vertex; 5] = [
    Vertex { position: (0.0,  0.0), tex_coords: (0.0, 0.0) },   // dummy vector because model indices start at 1

    Vertex { position: (-0.5, -0.5), tex_coords: (0.0, 0.0) }, // lower left
    Vertex { position: ( 0.5, -0.5), tex_coords: (0.0, 1.0) }, // lower right
    Vertex { position: ( 0.5,  0.5), tex_coords: (1.0, 1.0) }, // upper right
    Vertex { position: (-0.5,  0.5), tex_coords: (1.0, 0.0) }, // upper left
];

const INDICES: [u16; 6] = [
    1, 3, 4,
    1, 2, 3
];

pub fn positions(display: &GlutinFacade) -> VertexBuffer<Vertex> {
    VertexBuffer::new(display, &VERTICES).unwrap()
}

pub fn indices(display: &GlutinFacade) -> IndexBuffer<u16> {
    IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &INDICES
    ).unwrap()
}
