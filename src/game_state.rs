struct SharedState {
    data: u32
}

pub struct PlayingState {
    state: SharedState
}

pub trait GameState {
    fn input(&self) -> ();
    fn update(&self) -> ();
    fn draw(&self) -> ();
}

impl PlayingState {
    pub fn new(data: u32) -> PlayingState {
        PlayingState {
            state: SharedState { data: data }
        }
    }
}

impl GameState for PlayingState {
    fn input(&self) {
        println!("input, {}", &self.state.data)
    }

    fn update(&self) {
        println!("update, {}", &self.state.data)
    }

    fn draw(&self) {
        println!("draw, {}", &self.state.data)
    }
}
