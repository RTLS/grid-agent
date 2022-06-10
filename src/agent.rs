use crate::environment::{State, Position, Direction, Food};
use ndarray::{arr1, Array1};

const AGENT_STEP_ENERGY_COST: u32 = 1;
const AGENT_MAX_ENERGY: u32 = 100;

pub struct Agent {
    pub position: Position,
    pub energy: u32,
    pub alive: bool,
}

pub type Input = Array1<f32>;

pub enum AgentAction {
    Step(Direction),
}

impl Agent {
    pub fn new() -> Agent {
        Agent{
            position: Position{x: 50, y: 50},
            energy: AGENT_MAX_ENERGY,
            alive: true
        }
    }

    pub fn should_die(&self) -> bool {
        self.energy <= 0
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn eat_food(&mut self, food: &Food) {
       self.energy = std::cmp::min(AGENT_MAX_ENERGY, self.energy + food.energy); 
    }

    pub fn step_to(&mut self, position: Position) {
        self.position = position;
        self.energy -= AGENT_STEP_ENERGY_COST;
    }

    pub fn sensory_input(agent: &Agent, environment: &State) -> Input {
        arr1(&[])
    }

    pub fn preferred_action(agent: &Agent, input: &Input) -> AgentAction {
        AgentAction::Step(Direction::Right)
    }
}
