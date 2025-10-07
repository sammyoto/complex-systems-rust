mod agent; 
mod app;
use core::num;

use rand::prelude::*;
use app::App;
use eframe::egui;

fn main() -> eframe::Result {
    let num_agents: u32 = 1000;
    let world_x: f32 = 100.0;
    let world_y: f32 = 100.0;
    let neighborhood_radius: f32 = 2.0;

    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Diffusion Limited Aggregation",
        options,
        Box::new(|cc| Ok(Box::<App>::new(App::new(num_agents, world_x, world_y, neighborhood_radius)))),
    )
}
