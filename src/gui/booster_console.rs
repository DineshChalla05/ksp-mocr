use crate::flight_state::FlightState;
use crate::gui::console::Console;
use crate::gui::theme::{RETRO_AMBER, RETRO_RED};
use crate::telemetry::Telemetry;
use eframe::egui;

pub struct BoosterConsole;

impl Console for BoosterConsole {
    fn name(&self) -> &'static str {
        "BOOSTER"
    }

    fn show(&mut self, ui: &mut egui::Ui, telemetry: &Telemetry, flight_state: &FlightState) {
        let current_stage = telemetry.current_stage.get().unwrap_or(0);
        let throttle = telemetry.throttle.get().unwrap_or(0.0);
        let thrust = telemetry.thrust.get().unwrap_or(0.0);

        egui::Panel::left("booster_status_panel").show(ui, |ui| {
            ui.heading(egui::RichText::new("PROPULSION").color(RETRO_RED));
            ui.separator();
            ui.label(format!("Current stage: {}", current_stage));
            ui.label(format!("Thrust: {:.0} N", thrust));
            ui.label(format!(
                "Peak G-force: {:.2} G",
                flight_state.peak_g_force()
            ));
            ui.add_space(8.0);
            ui.label("Throttle:");
            ui.add(
                egui::ProgressBar::new(throttle)
                    .fill(RETRO_AMBER)
                    .show_percentage(),
            );
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(egui::RichText::new("EVENT LOG").color(RETRO_RED));
            ui.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for event in flight_state.events() {
                        ui.label(format!(
                            "T+{:.1}s  {}",
                            event.mission_elapsed_time, event.message
                        ));
                    }
                });
        });
    }
}
