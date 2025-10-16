mod agent; 
mod app;
use core::num;

use rand::prelude::*;
use app::App;
use agent::{Group, Agent};
use eframe::egui;

fn main() -> eframe::Result {
    let num_agents: u32 = 100;
    let world_x: f32 = 100.0;
    let world_y: f32 = 100.0;
    let neighborhood_radius: f32 = 20.0;
    let move_threshold: f32 = 0.5;

    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Boids",
        options,
        Box::new(|cc| Ok(Box::<App>::new(App::new(num_agents, world_x, world_y, neighborhood_radius, move_threshold)))),
    )
}

