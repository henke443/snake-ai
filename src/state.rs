use snake::Snake;
use piston::input::UpdateArgs;
use nalgebra::{Point2, Vector2};
use time::SteadyTime;
use input::Inputs;
use input;


pub struct WorldState {
    pub snakes: Vec<Snake>,
    pub inputs: Inputs,
    pub speed: f64,
    pub window_rect: Vector2<u32>,
    pub dt: f64,
    current_time: SteadyTime,
    accumulator: f64,
}

impl Default for WorldState {
    /// Gets the default world state.
    fn default() -> WorldState {
        WorldState {
            snakes: vec![Snake::new(Point2::new(200.0, 100.0), 10, 15.0),
                         Snake::new(Point2::new(500.0, 100.0), 30, 15.0),
                         Snake::new(Point2::new(500.0, 100.0), 30, 15.0),
                         Snake::new(Point2::new(500.0, 100.0), 30, 15.0),
                         Snake::new(Point2::new(500.0, 100.0), 30, 15.0)],
            inputs: Inputs::default(),
            speed: 1.0,
            dt: 0.01,
            window_rect: Vector2::new(0, 0),
            current_time: SteadyTime::now(),
            accumulator: 0.0,
        }
    }
}

impl WorldState {
    fn update_values(&mut self) {
        let inputs = input::get(self);

        let speed = self.speed * self.dt;

        for i in 0..self.snakes.len() {
            self.snakes[i].check_collision(&self.snakes);
            self.snakes[i].steer(100.0 * speed,
                                 5.0 * inputs.snake_steering[i] * speed,
                                 self.window_rect);
        }
    }

    /// This makes the speed become more constant, so that even if you lag or have a really good
    /// fps you will still hopefully (maybe) get the same behavior.
    /// I need to revisit this to see if it's the optimal approach...
    pub fn update(&mut self, args: &UpdateArgs) {

        self.dt = args.dt;

        //self.update_values();

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
