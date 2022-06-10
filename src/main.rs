extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::time::Instant;

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;

const UPDATES_PER_SECOND: u64 = 256;
const MAX_SPEED: bool = false;
const GRID_SIZE: u32 = 128;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const ENGLISH_VERMILLION: [f32; 4] = [211.0/256.0, 62.0/256.0, 67.0/256.0, 1.0];
const OLD_LAVENDER: [f32; 4] = [102.0/256.0, 99.0/256.0, 112.0/256.0, 1.0];
const FERN_GREEN : [f32; 4] = [88.0/256.0, 129.0/256.0, 87.0/256.0, 1.0];

mod agent;
mod environment;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    grid: graphics::grid::Grid,  // Grid
    environment: environment::State,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Grid
        let line = line::Line::new(BLACK, 0.0);

        // Agent
        let square = rectangle::square(0.0, 0.0, self.grid.units);
        let x = self.environment.agent.position.x as f64 * self.grid.units;
        let y = self.environment.agent.position.y as f64 * self.grid.units;

        // Food
        let food = &self.environment.food;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(OLD_LAVENDER, gl);

            // Grid
            self.grid.draw(&line, &c.draw_state, c.transform, gl);

            //Agent
            rectangle(ENGLISH_VERMILLION, square, c.transform.trans(x, y), gl);

            // Food
            for (position, _food) in food {
                let square = rectangle::square(0.0, 0.0, self.grid.units);
                let x = position.x as f64 * self.grid.units;
                let y = position.y as f64 * self.grid.units;
                rectangle(FERN_GREEN, square, c.transform.trans(x, y), gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.environment.update();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("kill bot", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: graphics::grid::Grid{cols: GRID_SIZE, rows: GRID_SIZE, units: 7.8},
        environment: environment::State::new(GRID_SIZE, GRID_SIZE),
    };

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = UPDATES_PER_SECOND;
    event_settings.ups = UPDATES_PER_SECOND;
    event_settings.bench_mode = MAX_SPEED;
    let mut events = Events::new(event_settings);

    let mut update_counts: u32 = 0;
    let mut started_at = Instant::now();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);

            update_counts += 1;
            if Instant::now().duration_since(started_at).as_millis() >= 1_000 {
                println!("Updates per second: {}", update_counts);
                started_at = Instant::now();
                update_counts = 0;
            }
        }
    }
}

