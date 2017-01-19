extern crate glium;
extern crate image;

use glium::texture::{RawImage2d,Texture2d};
use glium::backend::glutin_backend::GlutinFacade;
use std::path::Path;

pub fn load(filename: &str, display: &GlutinFacade) -> Texture2d {
    let image = image::open(&Path::new(filename)).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(
        image.into_raw(),
        image_dimensions
    );
    Texture2d::new(display, image).unwrap()
}
