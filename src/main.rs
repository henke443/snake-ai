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
use nalgebra::Vector2;

mod snake;
mod state;
mod input;

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
        window_rect: Vector2::new(width, height),
    };

    app.world_state.speed = 1.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
            // Simulate lag
            //std::thread::sleep(std::time::Duration::new(0, 1000_000_00));
        }
    }
}
