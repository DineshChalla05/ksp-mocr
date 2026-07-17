mod connection;
mod gui;
mod telemetry;

use crate::connection::init_connection;
use crate::gui::orbit_visualization::OrbitApp;
use crate::telemetry::get_telemetry;
use eframe::egui;

fn main() {
    let space_center = match init_connection() {
        Ok(sc) => sc,
        Err(err) => {
            println!("Failed to connect: {}", err);
            return;
        }
    };

    let telemetry = match get_telemetry(space_center) {
        Ok(tel) => tel,
        Err(err) => {
            println!("Failed to get telemetry: {}", err);
            return;
        }
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ksp-mocr",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            cc.egui_ctx.set_visuals(egui::Visuals { panel_fill: egui::Color32::BLACK, window_fill: egui::Color32::BLACK,
                // cool thing i found in the documentation for structs :3
                ..egui::Visuals::dark()
            });
            Ok(Box::new(OrbitApp {telemetry}))
        }),
    ).unwrap();
}
