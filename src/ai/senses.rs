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

/// Finds minimum value in a vector, neded for get_vision()
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


/// Casts a single ray for a snake and return an array containing "vision values" for all seeable
/// values. Vision values are 1 if the seen object is close, and -1 if it didn't see the object.
/// Index 0 in the vision values is for food_hit and index 1 is for wall_hit.
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

    // Each ray makes 2 float values, for each visible object.
    // Rays can pass through objects so need to make a vector of hits and
    // then only use the one that is closest to to the snake.
    // let mut snake_hits = Vec::new();
    let mut food_hits = Vec::new();
    let mut wall_hit = -1.0f64; // Cant hit several walls so wont need to check.

    let wr = Vector2::new((world.window_rect[0] / 2) as f64,
                          (world.window_rect[1] / 2) as f64);

    // Cuboid::new wants a vector2 of the cubes half extents, thats why I divide by 2.
    let window = Cuboid::new(Vector2::new(wr[0], wr[1]));
    // We need to move the windows origin to the center of the screen.
    let window_transform = Isometry2::new(Vector2::new(wr[0], wr[0]), 0.0);

    // Check if the bounds of the window transformed by window_transform is hit by our ray.
    // returns None if there was no hit or Some(toi) with a time of impact of the ray, less
    // value meaning the object is closer to the window.
    if let Some(toi) = window.toi_with_ray(&window_transform, &ray, false) {
        // False here means non-solid cast; basically math speak for a ray that can be cast from
        // within a object.
        wall_hit = 1.0 - (toi as f64 / 2.0);
        if wall_hit > 1.0 {
            wall_hit = 1.0;
        } else if wall_hit < -1.0 {
            wall_hit = -1.0;
        }
    }

    for _snake in &world.snakes {
        // For each snake
        if _snake.parts[0].origin != snake.parts[0].origin {
            // ... that is not self
            for part in &_snake.parts {
                // ... loop through each part of snake, and see if it's food.
                if part.is_food {
                    // ... see if food colides with ray.
                    let ball = Ball::new(part.radius); // Food is represented as a 2d ball.
                    let ball_transform = Isometry2::new(Vector2::new(part.origin.x, part.origin.y),
                                                        0.0);

                    // Gets time of impact of ray to ball transformed by ball_transform.
                    // If ray doesn't hit ball, it returns None.
                    if let Some(toi) = ball.toi_with_ray(&ball_transform, &ray, true) {
                        food_hits.push(toi);
                    }
                }
            }
        }
    }

    // Find smallest toi's in hits and use those,
    // preventing snake to have x-ray vision.
    // A value of 2.0 should be treated as -1, a value of 0 should be treated as 1

    let mut food_hit = -1.0;



    if !food_hits.is_empty() {
        //println!("wall_hit {}", wall_hit);

        food_hit = 1.0 - min_val(food_hits) / 1.2;

        if food_hit > 1.0 {
            food_hit = 1.0;
        } else if food_hit < -1.0 {
            food_hit = -1.0;
        }

    }

    [food_hit, wall_hit] //snake_hit]
}
