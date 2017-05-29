use snake::Snake;
use state::WorldState;
use nalgebra::{Point2, Vector2};
use ncollide::shape::{Cuboid, Ball};
use ncollide::query::{Ray, RayCast};
use nalgebra::Isometry2;
use std;

#[allow(unused)]
/// Return a vector of all senses experienced by snake: &Snake in the world: &WorldState
pub fn get(snake: &Snake, world: &WorldState) -> Vec<f64> {
    // If I need more senses just concatenate the Vec<f64> from that additional sense.
    get_vision(snake, world)
}

/// Needed for get_vision()
fn min_val(vector: Vec<f64>) -> f64 {
    let mut min_val = vector[0];

    for val in &vector {
        if *val < min_val {
            min_val = *val;
        }
    }

    min_val
}

#[allow(unused)]
/// Get vision for a &Snake in a &WorldState by ray tracing
fn get_vision(snake: &Snake, world: &WorldState) -> Vec<f64> {

    let mut vision: Vec<f64> = Vec::new();

    let head = &snake.parts[0];

    let head_point = Point2::new(head.origin.x, head.origin.y);

    let fov = 0.6 * std::f64::consts::PI; // ~110 degrees
    let mut rotation = head.rotation - fov / 2.0;
    let rotation_inc = fov / 16.0;

    // Spawn 16 rays.
    for i in 0..16 {
        // You can read this part like a poetry slam and perform it high.
        let hits = cast_ray(snake, world, &rotation, &head_point);
        for hit in hits.iter() {
            vision.push(*hit);
        }
        rotation += rotation_inc;
    }

    vision
}

fn cast_ray(snake: &Snake,
            world: &WorldState,
            rotation: &f64,
            head_point: &Point2<f64>)
            -> [f64; 2] {
    // Get a point on a circle, in this case a point on a circle with radius 1.0
    // around our head origin rotated by the head rotation; aka our unit vector for the ray.
    let dir_x = head_point.x + 1.0 * rotation.cos(); // 1.0 being the radius
    let dir_y = head_point.y + 1.0 * rotation.sin();

    let ray_direction = Vector2::new(dir_x, dir_y); // Unit vector
    let ray_origin = Point2::new(head_point.x, head_point.y);
    let ray = Ray::new(ray_origin, ray_direction);

    // Each ray makes 3 float values, for each visible object.
    // Rays can pass through objects so need to make a vector of hits and
    // then only use the one that is closest to to the snake.
    //let mut snake_hits = Vec::new();
    let mut food_hits = Vec::new();
    let mut wall_hit = -1.0f64; // Cant hit several walls so wont need to check.

    let wr = Vector2::new((world.window_rect[0] / 2) as f64,
                          (world.window_rect[1] / 2) as f64);

    // Cuboid::new wants a vector2 of the cubes half extents, thats why I divide by 2.
    let window = Cuboid::new(Vector2::new(wr[0], wr[1]));
    // We need to move the windows origin to the center of the screen.
    let window_transform = Isometry2::new(Vector2::new(wr[0], wr[0]), 0.0);

    if let Some(toi) = window.toi_with_ray(&window_transform, &ray, false) {
        // false means non-solid cast.
        wall_hit = 1.0 - toi as f64;
    }

    for _snake in &world.snakes {
        // For each snake
        if _snake.parts[0].origin != snake.parts[0].origin {
            // ... that is not self
            for part in &_snake.parts {
                // ... loop through each part of snake,
                // ... and see if it colides with ray.
                let ball = Ball::new(part.radius);
                let ball_transform = Isometry2::new(Vector2::new(part.origin.x, part.origin.y),
                                                    0.0);
                // gets time of impact of ray to ball with transform ball_transform
                // if ray doesn't hit ball, it returns None.
                if let Some(toi) = ball.toi_with_ray(&ball_transform, &ray, true) {
                    //snake_hits.push(toi);
                    if part.is_food {
                        food_hits.push(toi);
                    }
                }
            }
        }
    }

    // Find smallest toi's in hits and use those,
    // preventing snake to have x-ray vision.
    // A value of 2.0 should be treated as -1, a value of 0 should be treated as 1
    // let mut snake_hit = -1.0;
    // if !snake_hits.is_empty() {
    //     // if toi is 0, snake_hit is 1, if toi is 1, snake_hit is 0;
    //     snake_hit = 1.0 - min_val(snake_hits);
    // }

    let mut food_hit = -1.0;
    if !food_hits.is_empty() {
        food_hit = 1.0 - min_val(food_hits);
    }

    [food_hit, wall_hit] //snake_hit]
}
