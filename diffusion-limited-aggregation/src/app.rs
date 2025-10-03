use eframe::egui::{self, Color32};
use strum::IntoEnumIterator;
use rand::prelude::*;
use std::{thread};
use egui::{Stroke};
use egui_plot::{Plot, PlotPoints, Points, Polygon};
use std::time::{Instant, Duration};
use crate::agent::{Agent, Group};

pub struct App {
    agents: Vec<Agent>,
    num_agents: u32,
    world_x: f32,
    world_y: f32,
    neighborhood_radius: f32,
    move_threshold: f32,
    simulation_speed: u64,
    is_simulation_active: bool,
    show_radius: bool,
    last_update: Instant
}

impl App {
    pub fn new(num_agents: u32, world_x: f32, world_y: f32, neighborhood_radius: f32, move_threshold: f32) -> Self {
        Self {
            agents: Vec::new(),
            num_agents: num_agents,
            world_x: world_x,
            world_y: world_y,
            neighborhood_radius: neighborhood_radius,
            move_threshold: move_threshold,
            simulation_speed: 20,
            is_simulation_active: false,
            show_radius: false,
            last_update: Instant::now()
        }
    }

    pub fn generate_agents(&mut self) {
        let mut rng = rand::rng();
        let mut agents: Vec<Agent> = Vec::new();
        for i in 0..self.num_agents {
            let group: Group = Group::iter().choose(&mut rng).unwrap();
            agents.push(
                        Agent{  group: group, 
                                x: rng.random_range(0.0..self.world_x), 
                                y: rng.random_range(0.0..self.world_y)}
                        )
        }
        self.agents = agents;
    }

    pub fn step(&mut self) {
        // Check if its a valid time instance to step
        let now = Instant::now();
        let interval = Duration::from_millis(1000/self.simulation_speed);

        if now - self.last_update >= interval {
            // Change time instance to now
            self.last_update = now;
            // Get a random agent
            let mut rng = rand::rng();
            let agent_index: usize = rng.random_range(0..self.agents.len());
            let selected_agent: &Agent = self.agents.get(agent_index).unwrap();

            // Find that agent's neighbor ratio
            let neighbors: Vec<Agent> = selected_agent.get_neighbors(&self.agents, self.neighborhood_radius);
            let same_group_ratio: f32 = selected_agent.check_neighbors_group_ratio(&neighbors);

            // Move to a random location if the ratio is less than the move threshold
            if same_group_ratio < self.move_threshold {
                let selected_agent_mut: &mut Agent = self.agents.get_mut(agent_index).unwrap();
                selected_agent_mut.move_to_new_location(rng.random_range(0.0..self.world_x), rng.random_range(0.0..self.world_y));
            }
        }
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
                    ui.label("Move Threshold");
                    ui.add(egui::Slider::new(&mut self.move_threshold, 0.0..=1.0));
                }
            );
            ui.horizontal(|ui|
                {
                    ui.label("Simulation Speed (steps/second)");
                    ui.add(egui::Slider::new(&mut self.simulation_speed, 0..=1000));
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
            ui.heading("Schelling Simulation");

            Plot::new("agents_plot")
            .data_aspect(1.0)           // lock aspect ratio so circles stay circles
            .allow_drag(false)      // prevent dragging/panning
            .allow_zoom(false)      // prevent zooming
            .allow_scroll(false)    // prevent scroll-wheel zoom
            //.set_margin_fraction(Vec2::ZERO) // remove "auto margins"
            .include_x(0.0).include_x(self.world_x) // fix X range
            .include_y(0.0).include_y(self.world_y) // fix Y range
            .show(ui, |plot_ui| {
                
                let mut red = vec![];
                let mut blue = vec![];
                // let mut green = vec![];
                // let mut yellow = vec![];

                for a in &self.agents {
                    let p = [a.x as f64, a.y as f64];
                    match a.group {
                        Group::Red => red.push(p),
                        Group::Blue => blue.push(p),
                        // Group::Green => green.push(p),
                        // Group::Yellow => yellow.push(p)
                    }
                }
                
                if !red.is_empty() {
                    plot_ui.points(
                        Points::new("", PlotPoints::from(red)).color(Color32::RED).radius(3.0)
                    );
                }
                if !blue.is_empty() {
                    plot_ui.points(
                        Points::new("", PlotPoints::from(blue)).color(Color32::BLUE).radius(3.0)
                    );
                }
                // if !green.is_empty() {
                //     plot_ui.points(
                //         Points::new("", PlotPoints::from(green)).color(Color32::GREEN).radius(3.0)
                //     );
                // }
                // if !yellow.is_empty() {
                //     plot_ui.points(
                //         Points::new("", PlotPoints::from(yellow)).color(Color32::YELLOW).radius(3.0)
                //     );
                // }

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
