use snake::Snake;
use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use ai::DNA;
use nalgebra::Point2;

pub const MUTATE_RATE: f32 = 0.0008;

fn crossover(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    let len = b1.len();
    if len != b2.len() {
        panic!("Bytes didn't match in length!");
    }

    let between = Range::new(0, len - 1);
    let mut rng = rand::thread_rng();

    let cut_pos = between.ind_sample(&mut rng);

    let mut result = Vec::new();
    for i in 0..cut_pos {
        result.push(b1[i]);
    }
    for i in cut_pos..len {
        result.push(b2[i]);
    }
    if result.len() != len {
        panic!("Result didn't match original length.");
    }

    result
}

/// TODO, optimize this.
fn mutate(bytes: &mut [u8]) {
    let mut rng = rand::thread_rng();
    for byte in bytes {
        // Skip half randomly
        if rng.gen() {
            let should_mutate: u32 = Range::new(0, (1.0 / MUTATE_RATE) as u32).ind_sample(&mut rng);
            if should_mutate == 1 {

                let rand_byte: u8 = 1 << (Range::new(0, 8).ind_sample(&mut rng) as u8);

                //let old_byte = *byte;

                *byte = *byte ^ rand_byte; // Exclusive OR

                //println!("byte was: {:b} now is {:b}", old_byte, *byte);
            }
        }
    }
}

pub fn breed(s1: &Snake, s2: &Snake) -> Snake {

    let b1 = s1.dna.get_bytes();
    let b2 = s2.dna.get_bytes();

    let mut b3 = crossover(b1.as_slice(), b2.as_slice());

    mutate(b3.as_mut_slice());

    let dna = DNA::from_bytes(b3.as_slice());

    let between = Range::new(0.0f64, 500.0f64);
    let mut rng = rand::thread_rng();
    let rand_point = Point2::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng));

    let mut snake = Snake::new(rand_point, 3, s1.parts[0].radius * 2.0);

    snake.brain = dna.to_network();


    print!("New snake born!\nHash: {:x}\nColor: {:?}\n____________________\n\n",
           snake.dna.get_hash(),
           snake.base_color);

    snake
}
