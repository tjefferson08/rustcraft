extern crate glium;
extern crate image;

use glium::texture::{RawImage2d, Texture2d};
use glium::Display;
use std::path::Path;

pub fn load(filename: &str, display: &Display) -> Texture2d {
    let image = image::open(&Path::new(filename)).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    Texture2d::new(display, image).unwrap()
}

// square textures is implied here
pub struct Atlas {
    texture_size: u32,
    image_size: u32,
}

// CCW from upper right
pub type TextureCoordinates = [(f32, f32); 4];

impl Atlas {
    pub fn new(texture_size: u32, image_size: u32) -> Atlas {
        Atlas {
            texture_size: texture_size,
            image_size: image_size,
        }
    }

    fn textures_per_row(&self) -> u32 {
        self.image_size / self.texture_size
    }

    pub fn texture_coords_for(&self, x: u32, y: u32) -> TextureCoordinates {
        let unit_size = 1.0 / self.textures_per_row() as f32;
        let x_min = (x as f32) * unit_size;
        let y_min = (y as f32) * unit_size;
        let x_max = x_min + unit_size;
        let y_max = y_min + unit_size;
        // return [
        //     (x_max, y_max),
        //     (x_min, y_max),
        //     (x_min, y_min),
        //     (x_max, y_min)
        // ];
        [(0.0, 1.0), (0.0, 0.0), (1.0, 1.0), (1.0, 0.0)]
    }
}
