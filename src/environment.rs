use std::vec::Vec;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

const INITIAL_FOOD_COUNT: u32 = 10_000;
const AGENT_STEP_ENERGY_COST: u32 = 1;
const AGENT_MAX_ENERGY: u32 = 100;
const FOOD_ENERGY: u32 = 100;

pub struct State {
    pub grid: Grid,
    pub agent: Agent,
    pub food: Vec<Food>,
}

impl State {
    pub fn new(rows: u32, cols: u32) -> State {
        let mut state = State { grid: Grid{rows, cols}, agent: Agent::new(), food: Vec::new()};
        state.generate_initial_food();
        state
    }

    pub fn update(&mut self) {
        if self.agent.alive {
            self.step_agent();

            if self.agent.should_die() {
                self.agent.die();
            }
        }
    }

    fn step_agent(&mut self) {
        // Stepping has an energy cost
        self.agent.energy -= AGENT_STEP_ENERGY_COST;

        // Choose random direction
        let mut rng = rand::thread_rng();
        let direction: Direction = rng.gen();
        let new_position = Position::increment(&self.agent.position, direction);

        if self.valid_position(&new_position) {
            self.agent.position = new_position;
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
        position.x >= 0 && position.y >= 0 && position.x < cols && position.y < rows
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

pub struct Grid {
    rows: u32,
    cols: u32,
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

pub struct Agent {
    pub position: Position,
    pub energy: u32,
    pub alive: bool,
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
    pub position: Position,
    pub energy: u32,
}

impl Food {
    fn new_in(grid: &Grid) -> Food {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..grid.cols) as i32;
        let y = rng.gen_range(0..grid.rows) as i32;

        Food{position: Position{x, y}, energy: FOOD_ENERGY}
    }
}
