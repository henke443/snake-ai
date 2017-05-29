use snake::Snake;
use rand;
//use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use ai::DNA;
use nalgebra::Point2;

//pub const MUTATE_RATE: f32 = 0.05;

pub fn crossover(f1: Vec<f64>, f2: Vec<f64>) -> Vec<f64> {
    let len = f1.len();
    if len != f2.len() {
        panic!("Vectors didn't match in length!");
    }

    let between = Range::new(0, len - 1);
    let mut rng = rand::thread_rng();
    let cut_pos = between.ind_sample(&mut rng);

    let mut result = Vec::new();
    for i in 0..cut_pos {
        result.push(f1[i]);
    }

    for i in cut_pos..len {
        result.push(f2[i]);
    }

    result
}

// TODO deprecate this?
// fn crossover(b1: &[u8], b2: &[u8]) -> Vec<u8> {
//     let len = b1.len();
//     if len != b2.len() {
//         panic!("Bytes didn't match in length!");
//     }
//
//     let between = Range::new(0, len - 1);
//     let mut rng = rand::thread_rng();
//
//     let cut_pos = between.ind_sample(&mut rng);
//
//     let mut result = Vec::new();
//     for i in 0..cut_pos {
//         result.push(b1[i]);
//     }
//     for i in cut_pos..len {
//         result.push(b2[i]);
//     }
//     if result.len() != len {
//         panic!("Result didn't match original length.");
//     }
//
//     result
// }

/// TODO, deprecate this? Replace with a mutate_floats func
// fn mutate(bytes: &mut [u8]) {
//     let mut rng = rand::thread_rng();
//     let mut num_mutations = 0;
//     let len = bytes.len(); // Only used for println
//
//     // Skip 4 bytes, then mutate 4 bytes, then repeat untill done
//     // This because the first bytes in a float has special meaning.
//     let mut i = 0;
//     while i < (len - 4) {
//         i += 4;
//         for j in 0..4 {
//             let should_mutate: u32 = Range::new(0, (1.0 / MUTATE_RATE) as u32)
//                  .ind_sample(&mut rng);
//             if should_mutate == 1 {
//                 num_mutations += 1;
//                 let rand_byte: u8 = 1 << (Range::new(0, 8).ind_sample(&mut rng) as u8);
//                 //let old_byte = *byte;
//                 bytes[i + j] = bytes[i + j] ^ rand_byte; // Exclusive OR
//                 //println!("byte was: {:b} now is {:b}", old_byte, *byte);
//             }
//         }
//     }
//
//     println!("{}/{} bytes mutated", num_mutations, len);
// }

pub fn mutate_dna(dna: &mut DNA, mutate_rate: f32) {

    let mut rng = rand::thread_rng();
    let mut num_mutations = 0;
    let chance = Range::new(0, (1.0 / mutate_rate) as u32);

    for float in dna.0.iter_mut() {
        if chance.ind_sample(&mut rng) == 1 {
            num_mutations += 1;
            // Mutation is simply a float in the range -1.0 to 1.0 raised to 2.
            // Raising it with two makes extreme changes less likely, but its not needed.
            let mutation = (Range::new(-1.0, 1.0).ind_sample(&mut rng) as f64).powi(2);
            //println!("Was {} now is {}", *float, *float + mutation);
            *float = *float + mutation;
        }
    }

    //println!("{}/{} weights mutated", num_mutations, dna.0.len());
}

// TODO optimize this
pub fn breed(s1: &Snake, s2: &Snake, mutate_rate: f32) -> Snake {

    let f1 = s1.dna.get();
    let f2 = s2.dna.get();
    let mut dna = DNA(crossover(f1, f2));

    // let b1 = s1.dna.get_bytes();
    // let b2 = s2.dna.get_bytes();
    //
    // let mut b3 = crossover(b1.as_slice(), b2.as_slice());
    //let mut bytes = f3.get_bytes();
    //mutate(bytes.as_mut_slice());
    mutate_dna(&mut dna, mutate_rate);

    //let dna = DNA::from_bytes(bytes.as_slice());

    let mut snake = Snake::new(Point2::new(0.0, 0.0), 3, s1.parts[0].radius * 2.0);

    snake.brain = dna.to_network();

    snake
}
