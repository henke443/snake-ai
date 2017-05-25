use snake::*;
use piston::input::UpdateArgs;
use std;
// use time;
// use std::process::Command;
use nalgebra::{Point2, Vector2};

use input::Input;
use input;

pub struct WorldState {
    pub snakes: Vec<Snake>,
    pub input: Input,
    pub speed: f64,
    pub window_rect: Vector2<u32>,
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
            input: Input::default(),
            speed: 1.0,
            window_rect: Vector2::new(0, 0),
        }
    }
}

impl WorldState {
    /// Get input fetches input from the right sources.
    fn get_input(&mut self) {}

    pub fn update(&mut self, args: &UpdateArgs, sync_speed: f64) {

        let inp = input::get(self);

        let speed = sync_speed * self.speed;

        let mut i = 0;
        // let mut to_delete = Vec::new();
        for snake in &mut self.snakes {
            // if snake.alive {
            snake.steer(300.0 * speed * args.dt,
                        inp.snake_steering[i] * speed,
                        self.window_rect);
            // } else {
            //     to_delete.push(i);
            // }
            i += 1;
        }
        // for i in to_delete {
        //     if i < self.snakes.len() {
        //         self.snakes.remove(i);
        //     }
        // }
    }
}
