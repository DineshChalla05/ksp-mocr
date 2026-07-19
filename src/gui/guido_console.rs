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

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(egui::RichText::new("ATTITUDE").color(RETRO_RED));
            ui.separator();
            ui.label(format!("Pitch: {:.1}°", pitch));
            ui.label(format!("Heading: {:.1}°", heading));
            ui.label(format!("Roll: {:.1}°", roll));
            ui.label(format!("Situation: {:?}", situation));
        });
    }
}
