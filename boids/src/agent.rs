use core::num;
use std::sync::{Arc};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Clone)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32
}

impl Agent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x: x, y: y, vx: 0.0, vy: 0.0}
    }
    pub fn get_neighbors(&self, agents: &Vec<Agent>, radius: Arc<f32>) -> Vec<Agent>{
        let mut neighbors: Vec<Agent> = Vec::new();
        
        for agent in agents {
            if (self.x - agent.x).powf(2.0) + (self.y - agent.y).powf(2.0) < radius.powf(2.0) && agent != self {
                neighbors.push(agent.clone());
            }
        }

        neighbors
    }
    
    pub fn update_position(&mut self, move_coeff: f32) {
        self.x = self.x + (self.vx * move_coeff);
        self.y = self.y + (self.vy * move_coeff);
    }
}