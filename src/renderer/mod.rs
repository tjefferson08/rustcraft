extern crate glium;
extern crate image;
extern crate cgmath;

use glium::{Frame,Program,Surface};
use model::{Model};
use camera::Camera;
use glium::backend::glutin_backend::GlutinFacade;
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

    pub fn draw(&mut self, model: &Model, camera: &Camera) {
        self.target.draw(
            &model.positions(self.display),
            &model.indices(self.display),
            &self.program,
            &uniform! {
                model_matrix: model.model_matrix(),
                view_matrix: camera.view_matrix(),
                projection_matrix: camera.projection_matrix()
            },
            &Default::default()
        ).unwrap()
    }

    pub fn update(self) -> () {
        self.target.finish().unwrap();
    }
}
