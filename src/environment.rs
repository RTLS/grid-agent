use crate::agent::{Agent, AgentAction};
use std::collections::HashMap;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

const INITIAL_FOOD_COUNT: u32 = 10_000;
const FOOD_ENERGY: u32 = 100;

pub struct State {
    pub grid: Grid,
    pub agent: Agent,
    pub food: HashMap<Position, Food>,
}

impl State {
    pub fn new(rows: u32, cols: u32) -> State {
        let grid = Grid{rows, cols};
        let food = State::generate_initial_food(&grid);

        State {
            grid: grid,
            agent: Agent::new(),
            food: food,
        }
    }

    pub fn update(&mut self) {
        if self.agent.alive() {
            self.agent_action();
        }
    }

    fn agent_action(&mut self) {
        let sensory_input = Agent::sensory_input(&self.agent, &self);
        match Agent::preferred_action(&self.agent, &sensory_input) {
            AgentAction::Step(direction) => {
                self.step_agent(direction);
            }
        }
    }

    fn step_agent(&mut self, direction: Direction) {
        let new_position = Position::increment(&self.agent.position, direction);

        if self.in_bounds(&new_position) {
            self.agent.step_to(new_position.clone());
        }

        // If food is at that location, eat it
        match self.food.remove(&new_position) {
            Some(food) => {
                self.agent.eat_food(&food);
            },
            _ => (),
        }

        self.agent.maybe_die();
    }

    fn in_bounds(&self, position: &Position) -> bool {
        let rows = self.grid.rows as i32;
        let cols = self.grid.cols as i32;
        position.x >= 0 && position.y >= 0 && position.x < cols && position.y < rows
    }

    fn generate_initial_food(grid: &Grid) -> HashMap<Position, Food> {
       let mut food = HashMap::new();

       for _ in 0..INITIAL_FOOD_COUNT {
           let position = grid.rand_position();
           food.entry(position).or_insert(Food::new());
       }

       return food;
    }
}

pub struct Grid {
    rows: u32,
    cols: u32,
}

impl Grid {
    fn rand_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.cols) as i32;
        let y = rng.gen_range(0..self.rows) as i32;
        Position{x, y}
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=7) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::UpLeft,
            5 => Direction::UpRight,
            6 => Direction::DownLeft,
            _ => Direction::DownRight,
        }
    }
}

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn increment(position: &Position, dir: Direction) -> Position {
        match dir {
            Direction::Up => { Position{x: position.x, y: position.y - 1} },
            Direction::Down => { Position{x: position.x, y: position.y + 1} },
            Direction::Left => { Position{x: position.x - 1, y: position.y} },
            Direction::Right => { Position{x: position.x + 1, y: position.y} },
            Direction::UpLeft => { Position{x: position.x - 1, y: position.y - 1} },
            Direction::UpRight => { Position{x: position.x + 1, y: position.y - 1} },
            Direction::DownLeft => { Position{x: position.x - 1, y: position.y + 1} },
            Direction::DownRight => { Position{x: position.x + 1, y: position.y + 1} },
        }
    }
}

pub struct Food {
    pub energy: u32,
}

impl Food {
    fn new() -> Food {
        Food{energy: FOOD_ENERGY}
    }
}
