mod senses;
use snake::Snake;
use state::WorldState;
use rand::{thread_rng, Rng};
use serde_json::{Value, Error};
use serde_json;

#[allow(dead_code)]
#[allow(unused)]
pub mod nn;

pub use self::nn::NN;

/// The structure of a neural network, had to create this here as its internals were private in
/// the RustNN crate.
#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub layers: Vec<Vec<Vec<f64>>>, // layers: [ layer: [ neuron: [ weights: [f64, ...], ...], ...] ...], ...] }
    pub num_inputs: u32,
}

/// DNA is the weights of the neural networks flattened in a vector, used in the genetic algorithm.
pub struct DNA(Vec<f64>);

/// The layout of the neural network. 4 layers, 48 inputs, 32 hidden nodes and 2 outputs.
/// The number of hidden nodes are changeable without modifying other things in the source.
pub const NN_LAYOUT: [u32; 4] = [48, 16, 16, 2];

/// Get a DNA strand with default values.
impl Default for DNA {
    fn default() -> DNA {
        // NN::new() starts with random weights. 48-16-16-2 network (4 layers).
        let mut net = NN::new(&NN_LAYOUT);
        // convert net to json, because layers are private in the struct :/
        let json_net = net.to_json();

        let mut dna: DNA = DNA(Vec::new());

        let v: NeuralNetwork = serde_json::from_str(&json_net).unwrap();

        // put a println on the json_net and a std::thread:sleep if you want to see how the
        // json structure looks.

        for layer in v.layers {
            for neuron in layer {
                for weight in neuron {
                    dna.push(weight);
                }
            }
        }

        dna
    }
}

impl DNA {
    pub fn get(&self) -> Vec<f64> {
        let v = &self.0;
        v.to_vec()
    }

    pub fn set(&mut self, v: Vec<f64>) {
        self.0 = v;
    }

    pub fn push(&mut self, v: f64) {
        self.0.push(v);
    }

    pub fn to_network(&self) -> NN {
        let mut network = NN::new(&NN_LAYOUT);

        let layers: Vec<Vec<Vec<f64>>> = Vec::new();
        let neurons: Vec<Vec<f64>> = Vec::new();
        let mut weights: Vec<f64> = Vec::new();

        for weight in &self.0 {
            weights.push(*weight);
        }

        // for i in &NN_LAYOUT {
        //     for j in 0..NN_LAYOUT[i] { // For each neuron
        //         for h in 0..4 {// For each weight in said neuron
        //         neuron.push(weights[j]) // Push weight
        //     }
        // }

        let num_inputs = NN_LAYOUT[NN_LAYOUT.len() - 1];

        network.layers = layers;
        network.num_inputs = num_inputs;

        network
    }
}


/// Gets the steering for a snake based on the snake struct reference passed in as an argument.
/// The network has to be trained before this function can be used, or else it panics (is this true?).
///
/// #Examples
///
/// ```
/// let snake: snake::Snake = snake::Snake::default();
/// let steering: f64 = get_steering(snake);
/// // Steering is now in the range -1.0 to 1.0
/// ```
pub fn get_steering(snake: &Snake, world: &WorldState) -> f64 {

    let senses = senses::get(snake, world);

    let results = snake.brain.run(&senses);
    let left = results[0];
    let right = results[1];

    let steering: f64 = right - left; // Left is negative value right is positive

    //println!("call({:?})\n\n", snake.dna.0);

    steering
}
