extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate time;
extern crate rand;
extern crate ncollide;
extern crate nalgebra;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use time::*;
use nalgebra::Vector2;

mod snake;
mod state;
mod input;
// use snake::*;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    world_state: state::WorldState,
    should_render: bool,
    sync_speed: f64,
    fps: u32,
    window_rect: Vector2<u32>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        if self.should_render {

            self.window_rect = Vector2::new(args.width, args.height);

            use graphics::*;

            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

            let world_state = &self.world_state;

            let viewport = args.viewport();

            self.gl
                .draw(viewport, |c, gl| {
                    // Clear the screen.
                    clear(BLACK, gl);

                    for snake in &world_state.snakes {
                        snake.render(&c, gl, &args);
                    }
                });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world_state.window_rect = self.window_rect;
        self.world_state.update(args, self.sync_speed);
    }
}

fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let (width, height) = (800, 600);

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake Game", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    // Create a new game and run it.
    let mut app: App = App {
        gl: GlGraphics::new(opengl),
        world_state: state::WorldState::default(),
        should_render: true,
        sync_speed: 1.0,
        fps: 120,
        window_rect: Vector2::new(width, height),
    };



    app.world_state.speed = 1.0;

    // let mut wait_sum: f64 = 0.0;
    // let mut wait_num: i32 = 0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {

            let start = SteadyTime::now();
            app.update(&u); // Fetch input and update world struct
            let end = SteadyTime::now();

            let dt = end - start;

            let app_fps = app.fps as f64;
            let mut fps: f64 = dt.num_microseconds().unwrap() as f64;
            if fps == 0.0 {
                fps = app_fps;
            } else {
                fps = 1_000_000.0 / fps;
            }
            if fps > app_fps {
                fps = app_fps;
            }

            app.sync_speed = (app.fps as f64) / fps;

            let to_sleep = Duration::microseconds(1000_000 / (app.fps as i64)) - dt;

            if to_sleep > Duration::milliseconds(1) {
                let result: std::time::Duration = match to_sleep.to_std() {
                    Ok(v) => v,
                    Err(_) => std::time::Duration::new(0, 0),
                };

                std::thread::sleep(result);
            }
        }
    }
}
