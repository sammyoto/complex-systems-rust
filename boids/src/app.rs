use eframe::egui::{self, Color32};
use rand::prelude::*;
use std::{thread};
use std::sync::{Arc};
use egui::{Stroke};
use egui_plot::{Plot, PlotPoints, Points, Polygon};
use std::time::{Instant};
use crate::agent::{Agent};

pub struct App {
    agents: Arc<Vec<Agent>>,
    num_agents: u32,
    world_x: f32,
    world_y: f32,
    neighborhood_radius: f32,
    move_coeff: f32,
    center_of_mass_coeff: f32,
    average_velocity_coeff: f32,
    repulsion_coeff: f32,
    simulation_speed: u64,
    is_simulation_active: bool,
    show_radius: bool
}

impl App {
    pub fn new(num_agents: u32, world_x: f32, world_y: f32, neighborhood_radius: f32, move_coeff: f32) -> Self {
        Self {
            agents: Arc::new(Vec::new()),
            num_agents: num_agents,
            world_x: world_x,
            world_y: world_y,
            neighborhood_radius: neighborhood_radius,
            move_coeff: move_coeff,
            center_of_mass_coeff: 1.0,
            average_velocity_coeff: 1.0,
            repulsion_coeff: 40.0,
            simulation_speed: 20,
            is_simulation_active: false,
            show_radius: false
        }
    }

    pub fn generate_agents(&mut self) {
        let mut rng = rand::rng();
        let mut agents: Vec<Agent> = Vec::new();
        for i in 0..self.num_agents {
            agents.push(Agent::new(rng.random_range(0.0..self.world_x), rng.random_range(0.0..self.world_y)));
        }
        self.agents = Arc::new(agents);
    }

    pub fn step(&mut self) {
        let mut handles = vec![];
        let mut new_velocities: Vec<(f32, f32)> = vec![];
        let mut new_agents: Vec<Agent> = vec![];
        let neighborhood_radius_arc = Arc::new(self.neighborhood_radius.clone());

        for i in 0..self.agents.len() {
            let thread_agents = Arc::clone(&self.agents);
            let thread_neighborhood_radius = Arc::clone(&neighborhood_radius_arc);
            let center_of_mass_coeff_arc = Arc::new(self.center_of_mass_coeff.clone());
            let average_velocity_coeff_arc = Arc::new(self.average_velocity_coeff.clone());
            let repulsion_coeff_arc = Arc::new(self.repulsion_coeff.clone());

            let handle = thread::spawn(move || {
                // Pick an agent
                let agent = thread_agents.get(i).unwrap();
                // Find their neighbors
                let neighbors = agent.get_neighbors(&thread_agents, thread_neighborhood_radius);
                // Find center of mass
                let avgx = thread_agents.iter().map(|a| a.x).sum::<f32>() / thread_agents.len() as f32;
                let avgy = thread_agents.iter().map(|a| a.y).sum::<f32>() / thread_agents.len() as f32;
                let center_of_mass_vec = (avgx - agent.x * *center_of_mass_coeff_arc, avgy - agent.y * *center_of_mass_coeff_arc);
                // Find average velocity
                let avgVx = thread_agents.iter().map(|a| a.vx).sum::<f32>() / thread_agents.len() as f32;
                let avgVy = thread_agents.iter().map(|a| a.vy).sum::<f32>() / thread_agents.len() as f32;
                let average_velocity_vec = (avgVx * *average_velocity_coeff_arc, avgVy * *average_velocity_coeff_arc);
                // Find repulsion vector
                let mut repulsion_vecs: Vec<(f32, f32)> = vec![];
                for neighbor in neighbors {
                    let mut repulsion: (f32, f32) = (0.0, 0.0);
                    let dx = agent.x - neighbor.x;
                    let dy = agent.y - neighbor.y;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq > 0.0 {
                        let inv_dist = 1.0 / dist_sq.sqrt();
                        repulsion.0 += dx * inv_dist * *repulsion_coeff_arc;
                        repulsion.1 += dy * inv_dist * *repulsion_coeff_arc;
                    }

                    repulsion_vecs.push(repulsion);
                }
                let avgRepulsionX = repulsion_vecs.iter().map(|a|a.0).sum::<f32>() / repulsion_vecs.len() as f32;
                let avgRepulsionY = repulsion_vecs.iter().map(|a|a.1).sum::<f32>() / repulsion_vecs.len() as f32;
                let average_repulsion_vec = (avgRepulsionX, avgRepulsionY);
                // Return sum of all vectors
                let newVx = center_of_mass_vec.0 + average_velocity_vec.0 + average_repulsion_vec.0;
                let newVy = center_of_mass_vec.1 + average_velocity_vec.1 + average_repulsion_vec.1;

                (newVx, newVy)
            });
            handles.push(handle);
        }

        // join handles and get new velocities
        for handle in handles {
            let vel = handle.join().unwrap();
            new_velocities.push(vel);
        }

        // make new agents
        for i in 0..self.agents.len() {
            let mut agent = self.agents[i].clone();
            agent.vx = new_velocities[i].0;
            agent.vy = new_velocities[i].1;
            agent.update_position(self.move_coeff);
            new_agents.push(agent);
        }

        // replace old agents with new
        self.agents = Arc::new(new_agents);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // RIGHT SIDE PANEL
        egui::SidePanel::right("Simulation Settings").show(&ctx, |ui| {
            ui.heading("Simulation Settings");
            ui.horizontal(|ui|
                {
                    ui.label("Agent Population");
                    ui.add(egui::Slider::new(&mut self.num_agents, 0..=100000));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("World X Length");
                    ui.add(egui::Slider::new(&mut self.world_x, 0.0..=10000.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("World Y Length");
                    ui.add(egui::Slider::new(&mut self.world_y, 0.0..=10000.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Neighborhood Radius");
                    ui.add(egui::Slider::new(&mut self.neighborhood_radius, 0.0..=1000.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Move Coefficient");
                    ui.add(egui::Slider::new(&mut self.move_coeff, 0.0..=1.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Center of Mass Coefficient");
                    ui.add(egui::Slider::new(&mut self.center_of_mass_coeff, 0.0..=1.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Average Velocity Coefficient");
                    ui.add(egui::Slider::new(&mut self.average_velocity_coeff, 0.0..=1.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Repulsion Coefficient");
                    ui.add(egui::Slider::new(&mut self.repulsion_coeff, 0.0..=100.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Show Neighborhood Radius");
                    ui.checkbox(&mut self.show_radius, "")
                }
            );
            ui.horizontal(|ui|
                {
                if ui.button("Start/Restart").clicked() {
                    self.generate_agents();
                    self.is_simulation_active = true;
                };
                if ui.button("Pause/Unpause").clicked() {
                    if self.is_simulation_active {
                        self.is_simulation_active = false;
                    } else {
                        self.is_simulation_active = true;
                    }
                }
                }
            );
        });

        // CENTRAL PANEL
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Boids Simulation");

            Plot::new("agents_plot")
            .data_aspect(1.0)           // lock aspect ratio so circles stay circles
            .allow_drag(false)      // prevent dragging/panning
            .allow_zoom(false)      // prevent zooming
            .allow_scroll(false)    // prevent scroll-wheel zoom
            //.set_margin_fraction(Vec2::ZERO) // remove "auto margins"
            .include_x(0.0).include_x(self.world_x) // fix X range
            .include_y(0.0).include_y(self.world_y) // fix Y range
            .show(ui, |plot_ui| {
                
                let mut agent_points = vec![];

                for a in self.agents.as_ref().iter() {
                    let p = [a.x as f64, a.y as f64];
                    agent_points.push(p);
                }
                
                if !agent_points.is_empty() {
                    plot_ui.points(
                        Points::new("", PlotPoints::from(agent_points)).color(Color32::RED).radius(3.0)
                    );
                }

            // get mouse in plot coordinates
            let radius = self.neighborhood_radius as f64;

            // build circle polygon in center of world coordinates
            if self.show_radius {
                let circle: PlotPoints = (0..100)
                .map(|i| {
                    let theta = (i as f64) / 100.0 * std::f64::consts::TAU;
                    [(self.world_x as f64 / 2.0) + radius * theta.cos(),
                     (self.world_y as f64 / 2.0) + radius * theta.sin()]
                })
                .collect::<Vec<_>>()
                .into();

                plot_ui.polygon(
                    Polygon::new("", circle)
                        .fill_color(Color32::from_rgba_premultiplied(100, 150, 255, 64)) // semi-transparent
                        .stroke(Stroke::new(1.0, Color32::BLUE)),
            );
            };
        });
        });

        // APP CODE
        if self.is_simulation_active {
            self.step();
        }

        // keep redrawing
        ctx.request_repaint();
    }
}
