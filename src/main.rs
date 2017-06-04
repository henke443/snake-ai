extern crate piston; // piston core
extern crate graphics; // piston graphics
extern crate glutin_window; // opengl context creation
extern crate opengl_graphics; // opengl binding
//extern crate find_folder; // for finding our assets folder.

extern crate time;
extern crate rand;

extern crate ncollide; // 2d/3d/nd collision detection stuff
extern crate nalgebra; // has some neat matrices, vectors and points

extern crate rustc_serialize; // for ai::nn

// for json
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


use piston::window::WindowSettings;
use piston::event_loop::*; // Generic eventloop
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use nalgebra::{Vector2};

mod snake;
mod state;
mod input;
mod ai;
mod food;
mod geometry;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    world_state: state::WorldState,
    should_render: bool,
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
        self.world_state.update(args);
    }
}

fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let (width, height) = (1280, 720);

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake Game", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .fullscreen(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app: App = App {
        gl: GlGraphics::new(opengl),
        world_state: state::WorldState::default(),
        should_render: true,
        window_rect: Vector2::new(width, height),
    };

    // You can change these
    app.world_state.speed = 20.0;
    app.world_state.snake_length = 3;
    for _ in 0..4 { // Try increasing this a lot and remove printlns.
        let snake = snake::Snake::new(geometry::random_point_within(app.window_rect), 3, 20.0);
        app.world_state.snakes.push(snake);
    }

    // Add 10 snakes. with default length 2 and width 10


    //default: .max_fps(60).ups(120)
    let mut events = Events::new(EventSettings::new()).max_fps(60).ups(120);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            // Simulate lag:
            // std::thread::sleep(std::time::Duration::new(0, 1000_000_00));
            app.update(&u);
        }
    }
}
