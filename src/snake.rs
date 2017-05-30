use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::*;
use std;
use nalgebra::{Point2, Vector2};
use state::WorldState;
use ai::DNA;
use ai::nn::NN;
use ai;
use time::SteadyTime;
use food::Food;
use geometry::Circle;
use geometry;

pub struct Snake {
    pub alive: bool,
    pub parts: Vec<Part>,
    pub dna: DNA,
    pub brain: NN,
    pub base_color: [f32; 4],
    pub score: u32,
    pub last_eaten: SteadyTime,
}

pub struct Part {
    pub origin: Point2<f64>,
    pub radius: f64,
    pub rotation: f64,
    pub is_food: bool,
}

impl Circle for Part {
    fn origin(&self) -> Point2<f64> {
        self.origin
    }
    fn radius(&self) -> f64 {
        self.radius
    }
}

#[allow(unused)]
impl Part {
    pub fn clamp_to(&mut self, window: Vector2<u32>) -> bool {
        let radius = self.radius;
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
    pub fn is_outside(&self, window: Vector2<u32>) -> bool {
        let radius = self.parts[0].radius;
        let head = self.parts[0].origin;
        let mut clamped = false;

        if head.x - radius <= 1.0 || head.x + radius >= window[0] as f64 {
            clamped = true;
        } else if head.y - radius <= 1.0 || head.y + radius >= window[1] as f64 {
            clamped = true;
        }

        clamped
    }

    pub fn set_pos(&mut self, p: Point2<f64>) {
        for part in self.parts.iter_mut() {
            part.origin = Point2::new(p.x, p.y); //+ part.radius * (i as f64));
        }
    }

    pub fn add_part(&mut self) {
        let len = self.parts.len();
        let last = len - 1;

        // If the last part of snake is food, make it not food.
        if self.parts[last].is_food {
            self.parts[last].is_food = false;
            self.parts[last].radius = self.parts[last - 1].radius // Inherit the radius
        }

        let (x, y) = (self.parts[last].origin.x, self.parts[last].origin.y);

        let rot = self.parts[last].rotation;

        // Food as twice the radius. And this part which we add now is food because
        // it's the last part.
        let radius = self.parts[last].radius * 3.0;

        self.parts
            .push(Part {
                      origin: Point2::new(x, y + radius),
                      rotation: rot,
                      radius: radius,
                      is_food: true,
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
                            radius: width / 2.0,
                            is_food: false, // Head is not food.
                        }],
            dna: DNA::default(),
            brain: NN::new(&ai::NN_LAYOUT),
            base_color: [1f32; 4],
            score: 0,
            last_eaten: SteadyTime::now(),
        };

        snake.base_color = snake.dna.to_color();

        for _ in 1..num {
            snake.add_part();
        }

        snake
    }

    // #[allow(unused)]
    // pub fn kill(&mut self) {
    //     self.alive = false;
    // }

    /// Steer snake.
    /// Rotation is in radians in the range -.5pi to .5pi and is oriented to the head
    pub fn steer(&mut self, speed: f64, _rot: f64, window: Vector2<u32>) {
        let mut rot = self.parts[0].rotation;
        rot += _rot;

        if rot > 2.0 * std::f64::consts::PI {
            rot = 0.0;
        }

        self.parts[0].rotation = rot;
        let old_p = self.parts[0].origin;
        self.mov_add(speed, rot, window);

        if format!("{}", self.parts[0].origin.x) == "NaN" {
            println!("Rotation was: {}, _rot was: {}", rot, _rot);
            panic!("Was {} now is {}", old_p.x, self.parts[0].origin.y);
        }
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
            let mut diameter = p[i].radius * 2.0;
            if i == p.len() - 1 {
                // If last element, being food and having larger radius.
                diameter = p[i].radius * 1.5; // dirty fix for it floating behind other parts.
                // TODO, find mathematical relation between factor here and food radius Serialize
                // use that.
            }
            let a = (p[i].origin.y - p[i - 1].origin.y).atan2(p[i].origin.x - p[i - 1].origin.x);
            p[i].origin.x = p[i - 1].origin.x + diameter * a.cos();
            p[i].origin.y = p[i - 1].origin.y + diameter * a.sin();
        }

    }

    /// if 'self' has eaten some snake in 'snakes', return the index of that snake.
    pub fn has_eaten<'a>(&self, world: &WorldState) -> Option<usize> {

        for (i, snake) in world.snakes.iter().enumerate() {
            if snake.parts[0].origin != self.parts[0].origin {
                // Dirty
                for part in &snake.parts {
                    if part.is_food {
                        if geometry::collision(&self.parts[0], part) {
                            return Some(i);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn set_length(&mut self, length: usize) {
        while self.parts.len() < length {
            self.add_part();
        }
    }

    #[allow(unused_variables)]
    pub fn render(&self, c: &context::Context, gl: &mut GlGraphics, args: &RenderArgs) {
        let parts = &self.parts;

        if parts.len() > 0 {
            let food_color = [1.0, 0.2, 0.2, 0.5];
            let base_color = self.base_color;
            let mut color = [1.0, 1.0, 1.0, 1.0];

            // Rough emulation of vision.
            // let mut rotation = -0.3 * std::f64::consts::PI;
            // let rot_inc = std::f64::consts::PI * 0.0375; //pi/16 * 0.6
            // for i in 0..16 {
            //     let ray = [0.0, 0.0, 300.0, 2.0];
            //     let ray_transform = c.transform
            //         .trans(parts[0].origin.x, parts[0].origin.y)
            //         .rot_rad(-parts[0].rotation + rotation);
            //
            //     rectangle([1.0, 1.0, 1.0, 0.2], ray, ray_transform, gl);
            //
            //     rotation += rot_inc;
            // }

            let len = parts.len();
            for i in 0..len {
                if i == 1 {
                    color = base_color;
                } else if i == len - 1 {
                    color = food_color;
                }
                let radius = parts[i].radius;
                let square = [0.0, 0.0, radius * 2.0, radius * 2.0];

                let (x, y) = (parts[i].origin.x, parts[i].origin.y);
                let transform = c.transform.trans(x - radius, y - radius);
                ellipse(color, square, transform, gl);
            }
        } else {
            panic!("parts.len() was < 1");
        }
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake::new(Point2::new(50.0, 50.0), 2, 50.0)
    }
}
