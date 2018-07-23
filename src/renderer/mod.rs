extern crate cgmath;
extern crate glium;
extern crate image;

use camera::Camera;
use glium::texture::Texture2d;
use glium::{Display, Frame, Program, Surface};
use model::Model;
use shaders;
use textures;

pub struct Master<'dis> {
    display: &'dis Display,
    tex: Texture2d,
    pub target: Frame,
    program: Program,
    // sampler: Sampler<
}

impl<'dis> Master<'dis> {
    pub fn new(display: &Display, target: Frame) -> Master {
        let vertex_shader_src = shaders::load("src/shaders/vertex_shader.glsl");
        let fragment_shader_src = shaders::load("src/shaders/fragment_shader.glsl");
        let texture = textures::load("src/textures/grass.png", display);
        // let texture = textures::load("src/textures/atlas.png", display);
        // let sampler = texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        Master {
            display: display,
            target: target,
            program: Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None)
                .unwrap(),
            tex: texture,
        }
    }

    pub fn clear(&mut self) -> () {
        self.target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
    }

    pub fn draw(&mut self, model: &Model, camera: &Camera) {
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                // choose CLOSER things to draw
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        self.target
            .draw(
                &model.positions(self.display),
                &model.indices(self.display),
                &self.program,
                &uniform! {
                    model_matrix: model.model_matrix(),
                    view_matrix: camera.view_matrix(),
                    projection_matrix: camera.projection_matrix(),
                    grass_tex_sampler: self.tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                },
                &draw_params,
            )
            .unwrap()
    }

    pub fn update(self) -> () {
        self.target.finish().unwrap();
    }
}
