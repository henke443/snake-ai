pub use nalgebra::{Point2, Vector2};
use rand;
use rand::distributions::{IndependentSample, Range};

/// Trait for a 2d ball.
pub trait Circle {
    fn origin(&self) -> Point2<f64>;
    fn radius(&self) -> f64;
}

/// Checks if a circle c1 collides with a circle c2. Circles must implement the Circle trait.
pub fn collision<A: Circle, B: Circle>(c1: &A, c2: &B) -> bool {
    if (c1.origin().x - c2.origin().x).powi(2) + (c1.origin().y - c2.origin().y).powi(2) <=
       (c1.radius() + c2.radius()).powi(2) {
        return true;
    }
    false
}

/// Generates a random point within a box.
pub fn random_point_within(window: Vector2<u32>) -> Point2<f64> {
    let mut rng = rand::thread_rng();
    let rx = Range::new(1.0, window[0] as f32 - 1.0);
    let ry = Range::new(1.0, window[1] as f32 - 1.0);

    Point2::new(rx.ind_sample(&mut rng) as f64,
                ry.ind_sample(&mut rng) as f64)
}
