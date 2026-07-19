use crate::flight_state::FlightState;
use crate::gui::booster_console::BoosterConsole;
use crate::gui::console::Console;
use crate::gui::fido_console::FidoConsole;
use crate::gui::guido_console::GuidoConsole;
use crate::gui::theme::RETRO_RED;
use crate::telemetry::Telemetry;
use eframe::{Frame, egui};
use egui::Ui;
use krpc_client::services::space_center::VesselSituation;

pub struct MocrApp {
    telemetry: Telemetry,
    flight_state: FlightState,
    consoles: Vec<Box<dyn Console>>,
    active: usize,
}

impl MocrApp {
    pub fn new(telemetry: Telemetry) -> Self {
        let consoles: Vec<Box<dyn Console>> = vec![
            Box::new(FidoConsole),
            Box::new(BoosterConsole),
            Box::new(GuidoConsole),
        ];
        Self {
            telemetry,
            flight_state: FlightState::init_flight_state(),
            consoles,
            active: 0,
        }
    }
}

impl eframe::App for MocrApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui.ctx().request_repaint();

        let met = self.telemetry.met.get().unwrap_or(0.0);
        let current_stage = self.telemetry.current_stage.get().unwrap_or(0);
        let situation = self
            .telemetry
            .situation
            .get()
            .unwrap_or(VesselSituation::PreLaunch);
        let dynamic_pressure = self.telemetry.dynamic_pressure.get().unwrap_or(0.0);
        let thrust = self.telemetry.thrust.get().unwrap_or(0.0);
        let g_force = self.telemetry.g_force.get().unwrap_or(0.0);
        self.flight_state.tick(
            met,
            current_stage,
            situation,
            dynamic_pressure,
            thrust,
            g_force,
        );

        egui::Panel::top("flight_director_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("MOCR").color(RETRO_RED));
                ui.separator();
                ui.label(format!("MET: T+{:.0}s", met));
            });
        });

        egui::Panel::top("console_tabs").show(ui, |ui| {
            ui.horizontal(|ui| {
                for (i, console) in self.consoles.iter().enumerate() {
                    if ui
                        .selectable_label(self.active == i, console.name())
                        .clicked()
                    {
                        self.active = i;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            self.consoles[self.active].show(ui, &self.telemetry, &self.flight_state);
        });
    }
}
