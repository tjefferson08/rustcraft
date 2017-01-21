extern crate glium;

use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;

pub struct Window {
    display: GlutinFacade
}

impl Window {
    pub fn is_open(&self) -> bool {
        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return false,
                _ => ()
            }
        }

        true
    }

    pub fn display(&self) -> &GlutinFacade {
        &self.display
    }
}

pub fn new() -> Window {
    Window {
        display: glium::glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_depth_buffer(24)
            .build_glium()
            .unwrap()
    }
}
