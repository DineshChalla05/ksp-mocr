use crate::flight_state::FlightState;
use crate::telemetry::Telemetry;
use eframe::egui;

pub trait Console {
    // I genuinely hate strings in Rust cause wtf man, &str doesnt work :,/
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui, telemetry: &Telemetry, flight_state: &FlightState);
}
