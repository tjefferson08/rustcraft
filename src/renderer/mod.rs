extern crate glium;
extern crate image;

pub struct Master {
}

impl Master {
    pub fn new() -> Master {
        Master {
        }
    }

    pub fn clear(&self) -> () {
    }

    pub fn update(&self) -> () {
        println!("master render update");
    }

    pub fn draw(&self) -> () {
        println!("master render draw");
    }
}
