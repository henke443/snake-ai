use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::*;
use nalgebra::{Point2, Vector2};
use geometry;
use geometry::Circle;

pub struct Food {
    pub radius: f64,
    pub origin: Point2<f64>,
}

impl Circle for Food {
    fn origin(&self) -> Point2<f64> {
        self.origin
    }
    fn radius(&self) -> f64 {
        self.radius
    }
}

impl Food {
    pub fn new(origin: Point2<f64>, radius: f64) -> Food {
        Food {
            radius: radius,
            origin: origin,
        }
    }

    pub fn render(&self, c: &context::Context, gl: &mut GlGraphics, args: &RenderArgs) {
        let food_color = [1.0, 0.2, 0.2, 0.5];

        let radius = self.radius;
        let square = [0.0, 0.0, radius * 2.0, radius * 2.0];

        let (x, y) = (self.origin.x, self.origin.y);
        let transform = c.transform.trans(x - radius, y - radius);
        ellipse(food_color, square, transform, gl);
    }


    /// Create a new piece of food and spawn it within a window.
    pub fn new_within(window: Vector2<u32>) -> Food {
        Food::new(geometry::random_point_within(window), 10.0)
    }
}
