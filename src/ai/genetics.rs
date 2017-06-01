use snake::Snake;
use rand;
use rand::distributions::{IndependentSample, Range};
use ai::DNA;
use geometry;
use geometry::Point2;

#[allow(unused)]
pub fn crossover(s1: &Snake, s2: &Snake) -> DNA {
    let len = s1.dna.0.len();
    if len != s1.dna.0.len() {
        panic!("Vectors didn't match in length!");
    }

    let between = Range::new(0, len - 1);
    let mut rng = rand::thread_rng();
    let cut_pos = between.ind_sample(&mut rng);

    let mut result = Vec::new();
    for i in 0..cut_pos {
        result.push(s1.dna.0[i]);
    }

    for i in cut_pos..len {
        result.push(s1.dna.0[i]);
    }

    DNA(result)
}

/// A higher bias means score will be taken more into account when two snakes crossover.
pub fn crossover_biased(s1: &Snake, s2: &Snake, bias: f32) -> DNA {

    let len = s1.dna.0.len();
    if len != s1.dna.0.len() {
        panic!("Vectors didn't match in length!");
    }

    let between = Range::new(0, len - 1);
    let mut rng = rand::thread_rng();
    let mut cut_pos = between.ind_sample(&mut rng);

    let addition = ((s1.score - s2.score) as f32 * bias * (len as f32)) as i32;
    cut_pos = cut_pos + addition as usize;
    println!("real cut_pos was: {}", cut_pos);

    if cut_pos <= len {
        println!("cut_pos <= len == true");
    }
    if cut_pos <= len || cut_pos >= len {

        cut_pos = between.ind_sample(&mut rng);
    }

    println!("cut_pos was {} len was {} addition was {}",
             cut_pos,
             len,
             addition);

    let mut result = Vec::new();
    for i in 0..cut_pos {
        result.push(s1.dna.0[i]);
    }

    for i in cut_pos..len {
        result.push(s2.dna.0[i]);
    }

    DNA(result)
}

/// Randomly changes weights in dna based on the mutate_rate.
/// Lower mutate_rate means less mutations.
pub fn mutate(dna: &mut DNA, mutate_rate: f32) {

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

    println!("{}/{} weights mutated", num_mutations, dna.0.len());
}

/// Creates a new snake based on two snake and a mutate_rate.
/// Less mutate_rate means less mutations in the offspring DNA.
pub fn breed(s1: &Snake, s2: &Snake, mutate_rate: f32) -> Snake {

    //let mut dna = crossover(s1, s2);
    let mut dna = crossover_biased(s1, s2, 0.01);

    mutate(&mut dna, mutate_rate);

    let mut snake = Snake::new(Point2::new(0.0, 0.0), 3, s1.parts[0].radius * 2.0);

    snake.brain = dna.to_network();

    snake
}
