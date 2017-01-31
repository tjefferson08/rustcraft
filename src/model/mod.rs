pub use self::cube::Cube;

use glium::{IndexBuffer,VertexBuffer};
use glium::backend::glutin_backend::GlutinFacade;
use cgmath::Vector3;

mod cube;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: (f32, f32)
}

implement_vertex!(Vertex, position, tex_coords);

pub trait Model {
    fn update_position(&mut self, Vector3<f32>) -> ();
    fn update_rotation(&mut self, Vector3<f32>) -> ();
    fn positions(&self, &GlutinFacade) -> VertexBuffer<Vertex>;
    fn indices(&self, &GlutinFacade) -> IndexBuffer<u16>;
    fn model_matrix(&self) -> [[f32; 4]; 4];
}
