mod agent; 
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter; 
use agent::{Group, Agent};

fn generate_agents(num_agents: i16, world_x: f32, world_y: f32) -> Vec<Agent>{
    let mut rng = rand::rng();
    let mut agents: Vec<Agent> = Vec::new();
    for i in 0..num_agents {
        let group: Group = Group::iter().choose(&mut rng).unwrap();
        agents.push(
                    Agent{  group: group, 
                            x: rng.random_range(0.0..world_x), 
                            y: rng.random_range(0.0..world_y)}
                    )
    }
    agents
}
fn main() {
    let num_agents: i16 = 1000;
    let world_x: f32 = 100.0;
    let world_y: f32 = 100.0;
    let neighborhood_radius: f32 = 0.1;
    let move_threshold: f32 = 0.5;

    let mut agents: Vec<Agent> = generate_agents(num_agents, world_x, world_y);
    for agent in agents {
        println!("{:?}", agent);
    }
}
