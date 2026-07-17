use crate::telemetry::Telemetry;
use eframe::{Frame, egui};
use egui::Ui;
use std::f64::consts::PI;

pub struct OrbitApp {
    pub telemetry: Telemetry,
}

impl eframe::App for OrbitApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui.ctx().request_repaint();
        ui.heading("Orbit Visualization");

        let apoapsis = self.telemetry.apoapsis.get().unwrap_or(0.0);
        let periapsis = self.telemetry.periapsis.get().unwrap_or(0.0);
        let argument_of_periapsis = self.telemetry.argument_of_periapsis.get().unwrap_or(0.0);
        let true_anomaly = self.telemetry.true_anomaly.get().unwrap_or(0.0);
        let body_radius = self.telemetry.body_radius;

        let apoapsis_r = apoapsis + body_radius;
        let periapsis_r = periapsis + body_radius;
        let semi_major_axis = (apoapsis_r + periapsis_r) / 2.0;
        let eccentricity = (apoapsis_r - periapsis_r) / (apoapsis_r + periapsis_r);

        egui::Panel::left("telemetry_panel").show_inside(ui, |ui| {
            ui.heading("Telemetry");
            ui.separator();
            ui.label(format!("Apoapsis altitude: {:.0} m", apoapsis));
            ui.label(format!("Periapsis altitude: {:.0} m", periapsis));
            ui.label(format!("Semi major axis: {:.0} m", semi_major_axis));
            ui.label(format!("Eccentricity: {:.4}", eccentricity));
            ui.label(format!("Argument of periapsis: {:.2} rad", argument_of_periapsis));
            ui.label(format!("True anomaly: {:.2} rad", true_anomaly));
            ui.label(format!("Body radius: {:.0} m", body_radius));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Orbit View");
            draw_orbit(ui, apoapsis, periapsis, argument_of_periapsis, true_anomaly, body_radius);
        });
    }
}

fn draw_orbit(ui: &mut egui::Ui, apoapsis: f64, periapsis: f64, argument_of_periapsis: f64, true_anomaly: f64, body_radius: f64) {
    let painter = ui.painter();
    let screen = ui.max_rect();
    let center = screen.center();
    let apoapsis_r = apoapsis + body_radius;
    let periapsis_r = periapsis + body_radius;
    let semi_major_axis = (apoapsis_r + periapsis_r) / 2.0;
    let eccentricity = (apoapsis_r - periapsis_r) / (apoapsis_r + periapsis_r);
    let max_radius_px = (screen.width().min(screen.height()) / 2.0) as f64 * 0.9;
    let scale = max_radius_px / apoapsis_r;

    painter.circle_filled(center, (body_radius * scale) as f32, egui::Color32::from_rgb(120, 120, 255));

    let steps = 180;
    let mut points: Vec<egui::Pos2> = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let theta = (i as f64 / steps as f64) * 2.0 * PI;
        points.push(orbit_point(theta, semi_major_axis, eccentricity, argument_of_periapsis, center, scale));
    }
    painter.add(egui::Shape::line(points, egui::Stroke::new(1.5, egui::Color32::WHITE)));

    let ship_r = semi_major_axis * (1.0 - eccentricity.powi(2)) / (1.0 + eccentricity * true_anomaly.cos());
    let ship_angle = true_anomaly + argument_of_periapsis;
    let ship_pos = egui::pos2(
        center.x + (ship_r * scale * ship_angle.cos()) as f32,
        center.y - (ship_r * scale * ship_angle.sin()) as f32,
    );
    painter.circle_filled(ship_pos, 4.0, egui::Color32::YELLOW);
}

fn orbit_point(theta: f64, semi_major_axis: f64, eccentricity: f64, argument_of_periapsis: f64, center: egui::Pos2, scale: f64) -> egui::Pos2 {
    let r = semi_major_axis * (1.0 - eccentricity.powi(2)) / (1.0 + eccentricity * theta.cos());
    let angle = theta + argument_of_periapsis;
    egui::pos2(
        center.x + (r * scale * angle.cos()) as f32,
        center.y - (r * scale * angle.sin()) as f32,
    )
}
