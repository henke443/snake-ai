use rand::{thread_rng, Rng};
use state::WorldState;
use ai;

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

    // the ai module gets the steering of one snake based on that snakes vision.

    // This should get the values from the AI module soon to be made instead
    for i in 0..state.snakes.len() {
        let num: f64 = rng.gen_range(-5.0, 5.0);
        // Should be in the range of -1 to 1 but need more because it's random.
        // let num: f64 = -1.0;
        let steering: f64 = ai::get_steering(&state.snakes[i]);
        inp.snake_steering.push(steering);
    }

    inp
}
