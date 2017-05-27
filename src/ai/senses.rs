use snake::Snake;
use state::WorldState;
use nalgebra;


#[allow(unused)]
/// Return a vector of all senses experienced by snake: &Snake in the world: &WorldState
pub fn get(snake: &Snake, world: &WorldState) -> Vec<f64> {
    get_vision(snake, world)
}

#[allow(unused)]
fn get_vision(snake: &Snake, world: &WorldState) -> Vec<f64> {
    let mut vision = Vec::new();
    for _ in 0..48 {
        vision.push(1.0f64);
    }

    // Cast 16 rays from head
    // 3 groups makes 48 input neurons
    // Other snakes
    // Wall
    // Food

    use nalgebra::{Id, Point2, Vector2};
    use ncollide::shape::Cuboid;
    use ncollide::query::{Ray, RayCast};

    let cuboid = Cuboid::new(Vector2::new(1.0, 2.0));
    let ray_inside = Ray::new(nalgebra::origin::<Point2<f32>>(), Vector2::y());
    let ray_miss = Ray::new(Point2::new(2.0, 2.0), Vector2::new(1.0, 1.0));

    // Solid cast.
    assert_eq!(cuboid.toi_with_ray(&Id::new(), &ray_inside, true).unwrap(),
               0.0);

    // Non-solid cast.
    assert_eq!(cuboid.toi_with_ray(&Id::new(), &ray_inside, false).unwrap(),
               2.0);

    // The other ray does not intersect this shape.
    assert!(cuboid.toi_with_ray(&Id::new(), &ray_miss, false).is_none());
    assert!(cuboid.toi_with_ray(&Id::new(), &ray_miss, true).is_none());

    vision
}
