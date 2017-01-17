#[macro_use]
extern crate glium;

// mod teapot;
mod application;
mod window;
mod game_state;
mod renderer;
mod rectangle;

fn main() {
    let app = application::Application::new();
    app.run_game_loop();
}
