use nalgebra::Point2;

pub trait Circle {
    fn origin(&self) -> Point2<f64>;
    fn radius(&self) -> f64;
}

pub fn collision<T: Circle>(c1: &T, c2: &T) -> bool {
    if (c1.origin().x - c2.origin().x).powi(2) + (c1.origin().y - c2.origin().y).powi(2) <=
       (c1.radius() + c2.radius()).powi(2) {
        return true;
    }
    false
}
