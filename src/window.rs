extern crate glium;

use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::Display;

pub struct Window {
    display: Display,
}

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 768.0;

impl Window {
    pub fn new(events_loop: &EventsLoop) -> Window {
        let builder = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
            .with_title("Hello world");

        let context = ContextBuilder::new().with_depth_buffer(24);
        let display = Display::new(builder, context, events_loop).unwrap();
        display.gl_window().grab_cursor(true).unwrap();

        let w = Window {
            display: display
        };

        // w.display.get_window().unwrap().hide_cursor();
        // w.display
        //     .get_window()
        //     .unwrap()
        //     .set_cursor_position(WIDTH as i32 / 2, HEIGHT as i32 / 2);
        w
    }

    // Probably not needed (much) since `grab_cursor` keeps the cursor
    // in the display boundaries
    pub fn reset_cursor_position(&self) -> () {
        self.display
            .gl_window()
            .set_cursor_position(LogicalPosition::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0))
            .unwrap();
    }

    pub fn display(&self) -> &Display {
        &self.display
    }
}
