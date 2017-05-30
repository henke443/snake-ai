mod senses;
pub mod genetics;
use snake::Snake;
use state::WorldState;
use serde_json;

// Unsafe function for converting from f64 to [u8; 8] and back in this case.
use std::mem::transmute;

#[allow(dead_code)]
#[allow(unused)]
pub mod nn;

pub use self::nn::NN;

/// The structure of a neural network, had to create this here as its internals were private in
/// the RustNN crate.
#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
    // layers: [ layer: [ neuron: [ weights: [f64, ...], ...], ...] ...], ...] }
    pub layers: Vec<Vec<Vec<f64>>>,
    pub num_inputs: u32,
}

/// The layout of the neural can change and insert more or less hidden nodes but dont change
/// the 48 and the 2 in the ends, these are input and output nodes.
pub const NN_LAYOUT: [u32; 4] = [32, 16, 16, 3];

/// DNA is the weights of the neural networks flattened in a vector, used in the genetic algorithm.
pub struct DNA(Vec<f64>);

/// Get a DNA strand with default values.
impl Default for DNA {
    fn default() -> DNA {
        let net = NN::new(&NN_LAYOUT);
        // convert net to json, because layers are private in the struct :/
        let json_net = net.to_json();

        let mut dna: DNA = DNA(Vec::new());

        let v: NeuralNetwork = serde_json::from_str(&json_net).unwrap();

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



#[allow(unused)]
impl DNA {
    /// Simple hashing algorithm called djb2 which I can use to create a color
    /// from a DNA strand. http://www.cse.yorku.ca/~oz/hash.html
    pub fn get_hash(&self) -> u32 {
        // Need wrapping because hash will overflow several times.
        use std::num::Wrapping;
        let bytes = self.get_bytes();

        let mut hash = Wrapping(5381u32);
        for b in bytes {
            hash = ((hash << 5) + hash) + Wrapping(b as u32); /* hash * 33 + c */
        }
        hash.0
    }

    /// Converts the dna Vec<f64> to a longer Vec<u8> and returns it.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for float in self.get() {
            // Unsafe code incoming!
            let bytes_buffer: [u8; 8] = unsafe { transmute(float) }; // or .to_le()
            for byte in &bytes_buffer {
                bytes.push(*byte);
            }
        }
        bytes
    }

    /// Returns a copy of the dna vector.
    pub fn get(&self) -> Vec<f64> {
        let v = &self.0;
        v.to_vec()
    }

    /// Gets color for dna based on the `get_hash()` function.
    pub fn to_color(&self) -> [f32; 4] {
        let hash = self.get_hash();

        let r: f32 = ((hash & 0xFF0000) >> 16) as f32;
        let g: f32 = ((hash & 0x00FF00) >> 8) as f32;
        let b: f32 = (hash & 0x0000FF) as f32;

        [r / 256.0, g / 256.0, b / 256.0, 1.0]
    }

    /// Instead of self.0.push(v), because it's ugly.
    pub fn push(&mut self, v: f64) {
        self.0.push(v);
    }

    /// "Unflattens" dna to a network, this was really mindboggling to create.
    pub fn to_network(&self) -> NN {

        let mut network = NN::new(&NN_LAYOUT);

        let mut layers = Vec::new();

        // This loop is hard to understand :/ Wrote it in a caffeinated rampage.
        // Think its correct though.
        for i in 1..NN_LAYOUT.len() {
            let mut layer = Vec::new();
            for b in 0..NN_LAYOUT[i] {
                let mut neuron = Vec::new();
                let prev_num_inc = NN_LAYOUT[i - 1] + 1;
                for j in (b * prev_num_inc)..(b * prev_num_inc + prev_num_inc) {
                    neuron.push(self.0[j as usize]);
                }
                layer.push(neuron);
            }
            layers.push(layer);
        }

        let num_inputs = NN_LAYOUT[0];

        network.layers = layers;
        network.num_inputs = num_inputs;

        network
    }
}


/// Gets the steering for a snake based on the snake struct reference passed in as an argument.
/// The snake struct needs to have a NN field initialized.
///
/// #Examples
///
/// ```
/// let snake: snake::Snake = snake::Snake::default();
/// let steering: f64 = get_steering(snake);
/// // Steering is now in the range -1.0 to 1.0
/// ```
/// #Panics
/// Panics if steering is NaN, happened once so added it. Fixed so should never occur.
pub fn get_steering(snake: &Snake, world: &WorldState) -> [f64; 2] {

    let senses = senses::get(snake, world);

    let results = snake.brain.run(&senses);
    let left = results[0];
    let right = results[1];
    let speed = results[2];

    let steering: f64 = right - left; // Left is negative value right is positive

    // This is only true if steering is nan
    if steering != steering {
        panic!("Steering was NaN");
    }

    //let steering: f64 = 0.5;
    [steering, speed]
}
