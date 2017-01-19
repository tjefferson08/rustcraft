extern crate glium;
extern crate image;

use glium::{Frame,Program,Surface};
use rectangle::Rectangle;

pub struct Master {
    pub target: Frame
}

impl Master {
    pub fn new(target: Frame) -> Master {
        Master {
            target: target
        }
    }

    pub fn clear(&mut self) -> () {
        self.target.clear_color(0.0, 0.0, 1.0, 1.0);
    }

    pub fn draw(&mut self, rect: &Rectangle, program: &Program) {
        let
        self.target.draw(
            &rect.positions(),
            &rect.indices(),
            program,
            &uniform! {},
            &Default::default()
        ).unwrap()
    }

    pub fn update(self) -> () {
        self.target.finish().unwrap();
    }


}
