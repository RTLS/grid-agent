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
use std::vec::Vec;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};


const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;

const UPDATES_PER_SECOND: u64 = 16;
const MAX_SPEED: bool = false;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const ENGLISH_VERMILLION: [f32; 4] = [211.0/256.0, 62.0/256.0, 67.0/256.0, 1.0];
const OLD_LAVENDER: [f32; 4] = [102.0/256.0, 99.0/256.0, 112.0/256.0, 1.0];
const FERN_GREEN : [f32; 4] = [88.0/256.0, 129.0/256.0, 87.0/256.0, 1.0];

const INITIAL_FOOD_COUNT: u32 = 5000;
const AGENT_STEP_ENERGY_COST: u32 = 1;
const AGENT_MAX_ENERGY: u32 = 100;
const FOOD_ENERGY: u32 = 100;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

pub struct Position {
    x: i32, y: i32
}

impl Position {
    fn increment(position: &Position, dir: Direction) -> Position {
        match dir {
            Direction::Up => { Position{x: position.x, y: position.y - 1} },
            Direction::Down => { Position{x: position.x, y: position.y + 1} },
            Direction::Left => { Position{x: position.x - 1, y: position.y} },
            Direction::Right => { Position{x: position.x + 1, y: position.y} },
        }
    }
}

pub struct Agent {
    position: Position,
    energy: u32,
    alive: bool,
}

impl Agent {
    fn new() -> Agent {
        Agent{
            position: Position{x: 50, y: 50},
            energy: AGENT_MAX_ENERGY,
            alive: true
        }
    }

    fn should_die(&self) -> bool {
        self.energy <= 0
    }

    fn die(&mut self) {
        self.alive = false;
    }

    fn eat_food(&mut self, food: &Food) {
       self.energy = std::cmp::min(AGENT_MAX_ENERGY, self.energy + food.energy); 
    }
}

pub struct Food {
    position: Position,
    energy: u32,
}

impl Food {
    fn new_in(grid: &graphics::grid::Grid) -> Food {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..grid.cols) as i32;
        let y = rng.gen_range(0..grid.rows) as i32;

        Food{position: Position{x, y}, energy: FOOD_ENERGY}
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    grid: graphics::grid::Grid,  // Grid
    agent: Agent,
    food: Vec<Food>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Grid
        let line = line::Line::new(BLACK, 0.25);

        // Agent
        let square = rectangle::square(0.0, 0.0, self.grid.units);
        let x = self.agent.position.x as f64 * self.grid.units;
        let y = self.agent.position.y as f64 * self.grid.units;

        // Food
        let food = &self.food;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(OLD_LAVENDER, gl);

            // Grid
            self.grid.draw(&line, &c.draw_state, c.transform, gl);

            //Agent
            rectangle(ENGLISH_VERMILLION, square, c.transform.trans(x, y), gl);

            // Food
            for food in food {
                let square = rectangle::square(0.0, 0.0, self.grid.units);
                let x = food.position.x as f64 * self.grid.units;
                let y = food.position.y as f64 * self.grid.units;
                rectangle(FERN_GREEN, square, c.transform.trans(x, y), gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.agent.alive {
            self.step_agent();

            if self.agent.should_die() {
                self.agent.die();
            }
        }
    }

    fn step_agent(&mut self) {
        let mut rng = rand::thread_rng();
        let direction: Direction = rng.gen();
        let new_position = Position::increment(&self.agent.position, direction);
        if self.valid_position(&new_position) {
            self.agent.position = new_position;
            self.agent.energy -= AGENT_STEP_ENERGY_COST;
        } else if self.food_at_position(&new_position) {
            let index = self.food.iter()
                .position(|f| f.position.x == new_position.x && f.position.y == new_position.y)
                .unwrap();

            let food = self.food.swap_remove(index);
            self.agent.eat_food(&food);
            self.agent.position = new_position;
        }
    }

    fn valid_position(&mut self, position: &Position) -> bool {
        let in_bounds = self.in_bounds(position);
        let agent_position = position.x == self.agent.position.x && position.y == self.agent.position.y;
        let food_at_position = self.food_at_position(position);

        in_bounds && !agent_position && !food_at_position
    }

    fn in_bounds(&self, position: &Position) -> bool {
        let rows = self.grid.rows as i32;
        let cols = self.grid.cols as i32;
        position.x >= 0 && position.y >= 0 && position.x < cols - 1 && position.y < rows - 1
    }

    fn food_at_position(&self, position: &Position) -> bool {
        self.food.iter().any(|f| f.position.x == position.x && f.position.y == position.y)
    }

    fn generate_initial_food(&mut self) {
       self.food = Vec::with_capacity(INITIAL_FOOD_COUNT as usize);
       let mut food_count: u32 = 0;

       while food_count < INITIAL_FOOD_COUNT {
           let food = Food::new_in(&self.grid);
           if self.valid_position(&food.position) {
                self.food.push(food);
                food_count += 1;
           }
       }
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
        grid: graphics::grid::Grid{cols: 100, rows: 100, units: 10.0},
        agent: Agent::new(),
        food: Vec::new(),
    };

    app.generate_initial_food();

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = UPDATES_PER_SECOND;
    event_settings.ups = UPDATES_PER_SECOND;
    event_settings.bench_mode = MAX_SPEED;
    let mut events = Events::new(event_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

