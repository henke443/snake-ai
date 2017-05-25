use snake::*;
use piston::input::UpdateArgs;
use std;
use time;
//use std::process::Command;
use nalgebra::{Point2, Vector2};
use rand::{thread_rng, Rng};

pub struct Input {
    rotation: f64,
}

pub struct WorldState {
    pub snakes: Vec<Snake>,
    pub input: Input,
    pub speed: f64,
    pub window_rect: Vector2<u32>,
}

impl WorldState {
    pub fn get() -> WorldState {
        WorldState {
            snakes: vec![Snake::new(Point2::new(200.0, 100.0), 20, 15.0),
                         Snake::new(Point2::new(500.0, 100.0), 50, 15.0)],
            input: Input { rotation: 0.0 },
            speed: 1.0,
            window_rect: Vector2::new(0, 0),
        }
    }

    fn get_input(&mut self) {
        let mut rng = thread_rng();
        let num: f64 = rng.gen_range(-0.09 * std::f64::consts::PI, 0.09 * std::f64::consts::PI);
        self.input.rotation = num;
    }

    pub fn update(&mut self, args: &UpdateArgs, sync_speed: f64) {
        // ISSUE when two snakes die at the same time.
        let (x, y) = (self.window_rect.x, self.window_rect.y);

        self.get_input();
        let speed = sync_speed * self.speed;

        let mut len = 0;

        let mut i = 0;
        let mut to_delete = Vec::new();
        for snake in &mut self.snakes {
            if snake.alive {
                snake.steer(300.0 * speed * args.dt,
                            self.input.rotation * speed,
                            self.window_rect);
            } else {
                to_delete.push(i);
            }
            i += 1;
        }
        for i in to_delete {
            println!("Delete: {}", i);
            self.snakes.remove(i);
        }
    }
}
