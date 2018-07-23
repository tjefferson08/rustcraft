#[macro_use]
extern crate glium;
extern crate cgmath;

// mod teapot;
mod application;
mod camera;
mod constants;
mod entity;
mod game_state;
mod model;
mod renderer;
mod shaders;
mod textures;
mod window;

fn main() {
    let mut app = application::Application::new();
    app.run_game_loop();
}
