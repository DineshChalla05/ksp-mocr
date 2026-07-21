use crate::command::VesselControl;
use crate::flight_state::FlightState;
use crate::gui::booster_console::BoosterConsole;
use crate::gui::console::Console;
use crate::gui::fido_console::FidoConsole;
use crate::gui::guido_console::GuidoConsole;
use crate::gui::theme::{RETRO_AMBER, RETRO_RED};
use crate::session_guard::SessionGuard;
use crate::telemetry::Telemetry;
use eframe::{Frame, egui};
use egui::Ui;
use krpc_client::services::space_center::VesselSituation;

pub struct MocrApp {
    telemetry: Telemetry,
    flight_state: FlightState,
    session_guard: SessionGuard,
    discontinuity_notice: bool,
    vessel_control: VesselControl,
    staging_armed: bool,
    autopilot_armed: bool,
    autopilot_engaged: bool,
    target_pitch: f32,
    target_heading: f32,
    consoles: Vec<Box<dyn Console>>,
    active: usize,
}

impl MocrApp {
    pub fn new(telemetry: Telemetry, vessel_control: VesselControl) -> Self {
        let consoles: Vec<Box<dyn Console>> = vec![
            Box::new(FidoConsole),
            Box::new(BoosterConsole),
            Box::new(GuidoConsole),
        ];
        Self {
            telemetry,
            flight_state: FlightState::init_flight_state(),
            session_guard: SessionGuard::init_session_guard(),
            discontinuity_notice: false,
            vessel_control,
            staging_armed: false,
            autopilot_armed: false,
            autopilot_engaged: false,
            target_pitch: 90.0,
            target_heading: 90.0,
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

        if self.session_guard.tick(met) {
            self.staging_armed = false;
            self.autopilot_armed = false;
            if self.autopilot_engaged {
                let _ = self.vessel_control.disengage_autopilot();
            }
            self.autopilot_engaged = false;
            self.flight_state = FlightState::init_flight_state();
            self.discontinuity_notice = true;
        }

        self.flight_state.tick(
            met,
            current_stage,
            situation,
            dynamic_pressure,
            thrust,
            g_force,
        );

        if self.discontinuity_notice {
            egui::Panel::top("discontinuity_banner").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        RETRO_RED,
                        "⚠ SESSION DISCONTINUITY DETECTED — CONTROL DISARMED, FLIGHT LOG RESET",
                    );
                    if ui.button("ACKNOWLEDGE").clicked() {
                        self.discontinuity_notice = false;
                    }
                });
            });
        }

        egui::Panel::top("flight_director_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("MOCR").color(RETRO_RED));
                ui.separator();
                ui.label(format!("MET: T+{:.0}s", met));
                ui.separator();

                ui.checkbox(&mut self.staging_armed, "STAGING ARMED");
                let stage_clicked = ui
                    .add_enabled(self.staging_armed, egui::Button::new("ACTIVATE NEXT STAGE"))
                    .clicked();
                if stage_clicked {
                    if let Err(e) = self.vessel_control.activate_next_stage() {
                        eprintln!("Failed to activate next stage: {}", e);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.autopilot_armed, "AUTOPILOT ARMED");
                ui.separator();

                ui.label("Target pitch:");
                ui.add(
                    egui::DragValue::new(&mut self.target_pitch)
                        .range(-90.0..=90.0)
                        .suffix("°"),
                );
                ui.label("Target heading:");
                ui.add(
                    egui::DragValue::new(&mut self.target_heading)
                        .range(0.0..=360.0)
                        .suffix("°"),
                );

                let engage_clicked = ui
                    .add_enabled(self.autopilot_armed, egui::Button::new("ENGAGE"))
                    .clicked();
                if engage_clicked {
                    match self
                        .vessel_control
                        .engage_autopilot(self.target_pitch, self.target_heading)
                    {
                        Ok(()) => self.autopilot_engaged = true,
                        Err(e) => eprintln!("Failed to engage autopilot: {}", e),
                    }
                }

                let disengage_clicked = ui
                    .add_enabled(self.autopilot_engaged, egui::Button::new("DISENGAGE"))
                    .clicked();
                if disengage_clicked {
                    match self.vessel_control.disengage_autopilot() {
                        Ok(()) => self.autopilot_engaged = false,
                        Err(e) => eprintln!("Failed to disengage autopilot: {}", e),
                    }
                }

                if self.autopilot_engaged {
                    let error = self.vessel_control.autopilot_error().unwrap_or(0.0);
                    ui.separator();
                    ui.colored_label(RETRO_AMBER, format!("Pointing error: {:.1}°", error));
                }
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
