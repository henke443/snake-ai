use snake::Snake;
use snake;
use piston::input::UpdateArgs;
use nalgebra::{Point2, Vector2};
use time::SteadyTime;
use input::Inputs;
use input;
use ai;
use food::Food;
use geometry;

pub struct WorldState {
    pub snakes: Vec<Snake>,
    pub inputs: Inputs,
    pub speed: f64,
    pub window_rect: Vector2<u32>,
    pub dt: f64,
    pub global_best_score: u32,
    pub starve_time: u32,
    pub foods: Vec<Food>,
    pub snake_length: usize,

    // For the time-step (piston already has one but I layered this on top.)
    current_time: SteadyTime,
    accumulator: f64,
}

impl Default for WorldState {
    /// Gets the default world state.
    fn default() -> WorldState {
        WorldState {
            snakes: Vec::new(),
            snake_length: 3,
            inputs: Inputs::default(),
            speed: 1.0,
            dt: 0.01,
            window_rect: Vector2::new(0, 0),
            // for the time-step
            current_time: SteadyTime::now(),
            accumulator: 0.0,
            global_best_score: 0,
            starve_time: 4,
            foods: Vec::new(),
        }
    }
}

impl WorldState {
    fn update_values(&mut self) {
        let inputs = input::get(self);

        let speed = self.speed * self.dt;

        let mut to_kill = Vec::new();

        if self.snakes.len() < 2 {
            panic!("There's less than two snakes, can't breed!");
        }

        for i in 0..self.snakes.len() {
            self.snakes[i].steer(100.0 * speed * inputs.snake_speed[i],
                                 4.0 * inputs.snake_steering[i] * speed,
                                 self.window_rect);

            if self.snakes[i].is_outside(self.window_rect) {
                //to_kill.push(i); // Can enable this to kill on wall hit
            }

            if let Some(eaten) = self.snakes[i].has_eaten(self) {
                self.snakes[i].last_eaten = SteadyTime::now();
                for _ in 0..self.snake_length {
                    //self.snakes[eaten].parts.len() {
                    self.snakes[i].add_part();
                }
                self.snakes[i].score += 1;
                to_kill.push(eaten);
            }

            let now = SteadyTime::now();
            let time_since_eaten = (now - self.snakes[i].last_eaten).num_seconds();
            if time_since_eaten > self.starve_time as i64 {
                //println!("{} died of starvation", self.snakes[i].dna.get_hash());
                to_kill.push(i);
            }
        }

        // remove duplicates from to_kill
        to_kill.sort_by(|a, b| a.cmp(b));
        to_kill.dedup();

        for i in to_kill {

            self.snakes.remove(i);

            // Get two best snakes

            let mut max = 0;
            let mut second_max = 0;
            let mut best: usize = 0;
            let mut second_best: usize = 0;

            for (i, snake) in self.snakes.iter().enumerate() {
                if snake.score > max {
                    max = snake.score;
                    best = i;
                } else if snake.score > second_max {
                    second_max = snake.score;
                    second_best = i;
                }
            }

            // If the best snake has a score of 0, choose random snake.
            if max == 0 {
                use rand;
                use rand::distributions::{IndependentSample, Range};
                let mut rng = rand::thread_rng();
                let range = Range::new(0, self.snakes.len() - 1);

                best = range.ind_sample(&mut rng);
                second_best = range.ind_sample(&mut rng);

                //println!("Chose random snakes for breeding")
            }

            // If highscore...
            if max > self.global_best_score {
                self.global_best_score = max;
            }

            println!("Best alive: {}", max);

            println!("Global best: {}", self.global_best_score);

            // Random equation for mutate_rate and starve_time, can change this.
            let mutate_rate = 1.0 / (100.0 + self.global_best_score as f32 * 1.0);
            self.starve_time = self.global_best_score + 1;

            let mut child =
                ai::genetics::breed(&self.snakes[best], &self.snakes[second_best], mutate_rate);
            child.set_length(self.snake_length);
            child.set_pos(geometry::random_point_within(self.window_rect));

            self.snakes.push(child);
        }
    }

    /// This makes the speed become more constant, so that even if you lag or have a really good
    /// fps you will still hopefully (maybe) get the same behavior.
    /// I need to revisit this to see if it's the optimal approach...
    pub fn update(&mut self, args: &UpdateArgs) {
        self.dt = args.dt;

        // Uncomment this and comment the things below it to enable the default time-step only.
        //self.update_values();

        //Turns out piston already implements a time step in their event_loop.
        //However, I don't think that one is very good. Either that or I don't know how to fully
        //use it; the result being that the animation is somewhat choppy... should fix this soon.
        let new_time = SteadyTime::now();
        let frame_time = ((new_time - self.current_time).num_microseconds().unwrap() as f64) /
                         1000_000.0;
        self.current_time = new_time;
        self.accumulator += frame_time;

        while self.accumulator >= self.dt {
            self.update_values();
            self.accumulator -= self.dt;
        }
    }
}
