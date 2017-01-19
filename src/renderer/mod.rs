extern crate glium;
extern crate image;

use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;

pub struct Master {
    pub target: glium::Frame
}

impl Master {
    pub fn new(target: glium::Frame) -> Master {
        Master {
            target: target
        }
    }

    pub fn clear(&mut self) -> () {
        self.target.clear_color(0.0, 0.0, 1.0, 1.0);
    }

    pub fn draw(&mut self) -> () {
        // println!("master render draw");
        // target.draw(
        //     &positions,
        //     &indices,
        //     &program,
        //     &uniform! {},
        //     &Default::default()
        // ).unwrap();

    }

    pub fn update(self) -> () {
        // println!("master render update");
        self.target.finish().unwrap();
    }


}
