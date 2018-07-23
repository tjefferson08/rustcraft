extern crate glium;

use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;
use glium::DisplayBuild;

pub struct Window {
    display: GlutinFacade,
}

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 768.0;

impl Window {
    pub fn new() -> Window {
        let w = Window {
            display: glium::glutin::WindowBuilder::new()
                .with_dimensions(WIDTH as u32, HEIGHT as u32)
                .with_depth_buffer(24)
                .build_glium()
                .unwrap(),
        };
        w.display
            .get_window()
            .unwrap()
            .set_cursor_state(glutin::CursorState::Hide);
        w.display
            .get_window()
            .unwrap()
            .set_cursor_position(WIDTH as i32 / 2, HEIGHT as i32 / 2);
        w
    }

    pub fn reset_cursor_position(&self) -> () {
        self.display
            .get_window()
            .unwrap()
            .set_cursor_position(WIDTH as i32 / 2, HEIGHT as i32 / 2);
    }

    pub fn display(&self) -> &GlutinFacade {
        &self.display
    }
}
