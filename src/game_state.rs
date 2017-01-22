use std::vec::Vec;
use model::{Model,Rectangle};
use renderer::Master;

pub struct PlayingState {
    models: Vec<Box<Model>>
}

pub trait GameState {
    fn input(&self) -> ();
    fn update(&mut self) -> ();
    fn draw(&self, renderer: &mut Master) -> ();
}

impl PlayingState {
    pub fn new() -> PlayingState  {

        let mut playing_state = PlayingState {
            models: Vec::new(),
        };

        let mut rect1: Rectangle = Rectangle::new();
        let mut rect2: Rectangle = Rectangle::from_coords((1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        playing_state.models.push(
            Box::new(rect1)
        );
        playing_state.models.push(
            Box::new(rect2)
        );

        let immut_ps = playing_state;
        immut_ps
    }
}

impl GameState for PlayingState {
    fn input(&self) {
        // println!("input");
    }

    fn update(&mut self) {
        for mut model in self.models.iter_mut() {
            // model.update_position((0.01, 0.0, 0.0));
            model.update_rotation((0.01, 0.01, 0.0));
        }
    }

    fn draw(&self, renderer: &mut Master) {
        for  model in self.models.iter() {
            renderer.draw(
                model.as_ref()
            );
        }
    }
}
