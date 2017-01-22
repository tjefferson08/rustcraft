#[macro_use]
extern crate glium;
extern crate cgmath;

// mod teapot;
mod application;
mod window;
mod game_state;
mod renderer;
mod model;
mod shaders;
mod textures;
mod entity;

fn main() {
    let mut app = application::Application::new();
    app.run_game_loop();
}
