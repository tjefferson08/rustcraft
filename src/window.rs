extern crate glium;

use glium::DisplayBuild;
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin;

pub struct Window {
    display: GlutinFacade
}

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 768.0;

impl Window {
    pub fn new() -> Window {
        let w = Window {
            display: glium::glutin::WindowBuilder::new()
                .with_dimensions(1024, 768)
                .with_depth_buffer(24)
                .build_glium()
                .unwrap()
        };
        w.display.get_window().unwrap().set_cursor_state(glutin::CursorState::Hide);
        w
    }


    pub fn display(&self) -> &GlutinFacade {
        &self.display
    }

}
