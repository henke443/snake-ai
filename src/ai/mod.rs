mod senses;
use snake::Snake;

/// Gets the steering for a snake based on the snake struct passed in as an argument.
pub fn get_steering(snake: &Snake) -> f64 {
    let senses = senses::get(snake);
    //let result = get_values_from_network(senses), //not a real function
    // Result is most likely a vector with length 2 of some float from -1 to 1
    -1.0f64
}
