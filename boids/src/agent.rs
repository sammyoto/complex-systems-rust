use core::num;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self{
        Self{x, y}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Agent {
    pub velocity: Vec2,
    pub x: f32,
    pub y: f32,
}

impl Agent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {velocity: Vec2::new(0.0, 0.0), x, y}
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
        32.0
    }

    pub fn move_to_new_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}