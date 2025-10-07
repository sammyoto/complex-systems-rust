use core::num;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum State {
    Movable,
    Stationary
}

#[derive(Debug, PartialEq, Clone)]
pub struct Agent {
    pub state: State,
    pub x: f32,
    pub y: f32,
}

impl Agent {
    pub fn new(state: State, x: f32, y: f32) -> Self {
        Self {state, x, y}
    }
    pub fn get_neighbors(&self, agents: &Vec<Agent>, radius: f32) -> Vec<Agent>{
        let mut neighbors: Vec<Agent> = Vec::new();
        
        for agent in agents {
            if (self.x - agent.x).powf(2.0) + (self.y - agent.y).powf(2.0) < radius.powf(2.0) && agent != self {
                neighbors.push(agent.clone());
            }
        }

        neighbors
    }
    // return ratio of same group neighbors to total neighbors
    pub fn check_neighbors_for_stationary(&self, neighbors: &Vec<Agent>) -> bool {
        for neighbor in neighbors {
            if neighbor.state == State::Stationary {
                return true
            }
        }

        false
    }

    pub fn move_to_new_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}