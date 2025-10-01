use core::num;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum Group {
    Blue,
    Red,
    // Green,
    // Yellow
}

#[derive(Debug, PartialEq, Clone)]
pub struct Agent {
    pub group: Group,
    pub x: f32,
    pub y: f32,
}

impl Agent {
    pub fn new(group: Group, x: f32, y: f32) -> Self {
        Self {group, x, y}
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
    pub fn check_neighbors_group_ratio(&self, neighbors: &Vec<Agent>) -> f32 {
        let num_neighbors: f32 = neighbors.len() as f32;
        let mut same_count: u16 = 0;

        for neighbor in neighbors {
            if neighbor.group == self.group {
                same_count += 1;
            }
        }

        same_count as f32/num_neighbors
    }

    pub fn move_to_new_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}