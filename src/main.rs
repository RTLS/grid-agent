extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const ENGLISH_VERMILLION: [f32; 4] = [211.0/256.0, 62.0/256.0, 67.0/256.0, 1.0];
const OLD_LAVENDER: [f32; 4] = [102.0/256.0, 99.0/256.0, 112.0/256.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    grid: graphics::grid::Grid,  // Grid
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let line = line::Line::new(BLACK, 0.25);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(OLD_LAVENDER, gl);

            self.grid.draw(&line, &c.draw_state, c.transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // TODO
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: graphics::grid::Grid{cols: 100, rows: 100, units: 10.0}
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

