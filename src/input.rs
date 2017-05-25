use rand::{thread_rng, Rng};
use state::WorldState;
use std;

pub struct Input {
    pub snake_steering: Vec<f64>,
}

impl Default for Input {
    fn default() -> Input {
        Input { snake_steering: Vec::new() }
    }
}

pub fn get(state: &WorldState) -> Input {
    let mut rng = thread_rng();

    let mut inp = Input::default();

    // This should get the values from the AI module soon to be made instead
    for _ in 0..state.snakes.len() {
        let num: f64 = rng.gen_range(-0.09 * std::f64::consts::PI, 0.09 * std::f64::consts::PI);
        inp.snake_steering.push(num);
    }

    inp
}
