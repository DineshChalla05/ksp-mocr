use crate::flight_state::FlightState;
use crate::gui::console::Console;
use crate::gui::theme::RETRO_RED;
use crate::telemetry::Telemetry;
use eframe::egui;
use krpc_client::services::space_center::VesselSituation;

pub struct GuidoConsole;

impl Console for GuidoConsole {
    fn name(&self) -> &'static str {
        "GUIDO"
    }

    fn show(&mut self, ui: &mut egui::Ui, telemetry: &Telemetry, _flight_state: &FlightState) {
        let pitch = telemetry.pitch.get().unwrap_or(0.0);
        let heading = telemetry.heading.get().unwrap_or(0.0);
        let roll = telemetry.roll.get().unwrap_or(0.0);
        let situation = telemetry
            .situation
            .get()
            .unwrap_or(VesselSituation::PreLaunch);
        let period = telemetry.period.get().unwrap_or(0.0);
        let time_to_apoapsis = telemetry.time_to_apoapsis.get().unwrap_or(0.0);
        let time_to_periapsis = telemetry.time_to_periapsis.get().unwrap_or(0.0);

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(egui::RichText::new("ATTITUDE").color(RETRO_RED));
            ui.separator();
            ui.label(format!("Pitch: {:.1}°", pitch));
            ui.label(format!("Heading: {:.1}°", heading));
            ui.label(format!("Roll: {:.1}°", roll));
            ui.label(format!("Situation: {:?}", situation));
            ui.label(format!("Orbital period: {}", format_duration(period)));
            ui.label(format!("Time to apoapsis: {}", format_duration(time_to_apoapsis)));
            ui.label(format!("Time to periapsis: {}", format_duration(time_to_periapsis)));
        });
    }
}

fn format_duration(total_seconds: f64) -> String {
    let total_seconds = total_seconds.max(0.0) as i64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}