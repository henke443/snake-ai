use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::*;
use std;
use nalgebra::{Point2, Vector2};
use ncollide::shape::Ball;
use ai::DNA;
use ai::nn::NN;
use ai;

pub struct Snake {
    pub alive: bool,
    pub parts: Vec<Part>,
    pub dna: DNA,
    pub brain: NN,
}

pub struct Part {
    origin: Point2<f64>,
    ball: Ball<f64>,
    rotation: f64,
}

trait Circle {
    fn origin(&self) -> Point2<f64>;
    fn radius(&self) -> f64;
    fn rotation(&self) -> f64;
}

impl Circle for Part {
    fn rotation(&self) -> f64 {
        self.rotation
    }
    fn origin(&self) -> Point2<f64> {
        self.origin
    }
    fn radius(&self) -> f64 {
        self.ball.radius()
    }
}

impl Part {
    pub fn clamp_to(&mut self, window: Vector2<u32>) -> bool {
        let radius = self.radius();
        let mut clamped = false;
        if self.origin.x - radius <= 0.0 {
            self.origin.x = radius; // Correct
            clamped = true;
        } else if self.origin.x + radius > window[0] as f64 {
            self.origin.x = window[0] as f64 - radius; // Correct
            clamped = true;
        }
        if self.origin.y - radius <= 0.0 {
            self.origin.y = radius; //  Incorrect
            clamped = true;
        } else if self.origin.y + radius > window[1] as f64 {
            self.origin.y = window[1] as f64 - radius; // Correct
            clamped = true;
        }
        clamped
    }
}

impl Snake {
    pub fn add_part(&mut self) {
        let len = self.parts.len();

        let (x, y) = (self.parts[len - 1].origin.x, self.parts[len - 1].origin.y);

        let rot = self.parts[len - 1].rotation();

        let radius = self.parts[len - 1].radius(); //self.parts[len - 1].radius();

        self.parts
            .push(Part {
                      origin: Point2::new(x, y + radius * 2.0),
                      rotation: rot,
                      ball: Ball::new(radius),
                  });
    }

    /// Returns a new snake with head at position `p`, tail length of `num`
    /// and tail width of `width`
    pub fn new(p: Point2<f64>, num: i32, width: f64) -> Snake {
        let mut snake = Snake {
            alive: true,
            parts: vec![Part {
                            origin: Point2::new(p.x, p.y + width),
                            rotation: 0.0,
                            ball: Ball::new(width / 2.0),
                        }],
            dna: DNA::default(),
            brain: NN::new(&ai::NN_LAYOUT),
        };

        for _ in 1..num {
            snake.add_part();
        }

        snake
    }

    #[allow(unused)]
    pub fn kill(&mut self) {
        self.alive = false;
    }

    /// Steer snake.
    /// Rotation is in radians in the range -.5pi to .5pi and is oriented to the head
    pub fn steer(&mut self, speed: f64, _rot: f64, window: Vector2<u32>) {
        let mut rot = self.parts[0].rotation;
        rot += _rot;

        if rot > 2.0 * std::f64::consts::PI {
            rot = 0.0;
        }

        self.parts[0].rotation = rot;
        self.mov_add(speed, rot, window);
    }

    /// mov_add adds movement in a direction based on speed (length of added movement)
    /// and rot (rotation around the parts centerpoint signifying in which direction to apply force)
    /// rotation is in radians between -2*PI and 2*PI (-360 to 360 degrees)
    fn mov_add(&mut self, speed: f64, _rot: f64, window: Vector2<u32>) {
        let pi: f64 = std::f64::consts::PI;
        let pi2: f64 = pi * 2.0;
        let mut rot = _rot;

        if rot > pi2 || rot < -pi2 {
            rot = rot % pi2;
        }

        self.parts[0].origin.x += rot.cos() * speed;
        self.parts[0].origin.y -= rot.sin() * speed;
        self.parts[0].rotation = rot;

        self.parts[0].clamp_to(window);

        let p = &mut self.parts;


        // For each part that is not the head
        for i in 1..p.len() {
            let diameter = p[i].radius() * 2.0;
            let a = (p[i].origin.y - p[i - 1].origin.y).atan2(p[i].origin.x - p[i - 1].origin.x);
            p[i].origin.x = p[i - 1].origin.x + diameter * a.cos();
            p[i].origin.y = p[i - 1].origin.y + diameter * a.sin();
        }
    }

    pub fn check_collision(&self, snakes: &[Snake]) {
        let head_pos = self.parts[0].origin;
        for snake in snakes {
            if head_pos != snake.parts[0].origin {
                for i in 0..snake.parts.len() {
                    if snake.parts[i].origin == head_pos {
                        println!("Collision")
                    }
                }
            }
        }
    }

    #[allow(unused_variables)]
    pub fn render(&self, c: &context::Context, gl: &mut GlGraphics, args: &RenderArgs) {
        let parts = &self.parts;
        if parts.len() > 0 {
            let mut color = [1.0, 1.0, 1.0, 1.0];
            for i in 0..parts.len() {
                if i >= 1 {
                    color = [1.0,
                             1.0 / (i + 1) as f32,
                             1.0 / ((((i + 1) as f32) / 7.0)),
                             1.0];
                }

                let radius = parts[i].radius();
                let square = [0.0, 0.0, radius * 2.0, radius * 2.0];

                let (x, y) = (parts[i].origin.x, parts[i].origin.y);
                let transform = c.transform.trans(x - radius, y - radius);
                ellipse(color, square, transform, gl);
            }
        }
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake::new(Point2::new(50.0, 50.0), 5, 50.0)
    }
}
