use state::WorldState;
use ai;

pub struct Inputs {
    pub snake_steering: Vec<f64>,
    pub snake_speed: Vec<f64>,
}

impl Default for Inputs {
    fn default() -> Inputs {
        Inputs {
            snake_steering: Vec::new(),
            snake_speed: Vec::new(),
        }
    }
}

/// Gets all inputs and returns a Input struct containing them.
/// In the common case it fetches the input from the neural network but
/// you can easily make it get data from keyboard instead.
///
/// #Examples
///
/// ```
/// let inputs = input::get();
/// let steering: f64 = inputs.steering;
/// ```
pub fn get(world: &WorldState) -> Inputs {

    let mut inputs = Inputs::default();

    // This should get the values from the AI module soon to be made instead
    for i in 0..world.snakes.len() {
        // the ai module gets the steering of one snake based on that snakes senses.
        let steering = ai::get_steering(&world.snakes[i], world);

        inputs.snake_steering.push(steering[0]);
        inputs.snake_speed.push(steering[1]);

    }

    //println!("{:?}", inputs.snake_steering);

    inputs
}
