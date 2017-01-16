extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;

pub struct Window {
    pub display: GlutinFacade
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
}

pub fn new() -> Window {
    Window {
        display: glium::glutin::WindowBuilder::new()
            .build_glium()
            .unwrap()
    }
}
