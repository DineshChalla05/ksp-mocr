use crate::flight_state::FlightState;
use crate::gui::console::Console;
use crate::gui::theme::{RETRO_AMBER, RETRO_BLUE, RETRO_GREEN, RETRO_GRID, RETRO_RED};
use crate::telemetry::Telemetry;
use eframe::egui;
use std::f64::consts::PI;
use std::fmt::format;

pub struct FidoConsole;

impl Console for FidoConsole {
    fn name(&self) -> &'static str {
        "FIDO"
    }

    fn show(&mut self, ui: &mut egui::Ui, telemetry: &Telemetry, _flight_state: &FlightState) {
        let apoapsis = telemetry.apoapsis.get().unwrap_or(0.0);
        let periapsis = telemetry.periapsis.get().unwrap_or(0.0);
        let argument_of_periapsis = telemetry.argument_of_periapsis.get().unwrap_or(0.0);
        let true_anomaly = telemetry.true_anomaly.get().unwrap_or(0.0);
        let body_radius = telemetry.body_radius;
        let inclination = telemetry.inclination.get().unwrap_or(0.0);
        let surface_speed = telemetry.surface_speed.get().unwrap_or(0.0);
        let orbital_speed = telemetry.orbital_speed.get().unwrap_or(0.0);

        let apoapsis_r = apoapsis + body_radius;
        let periapsis_r = periapsis + body_radius;
        let semi_major_axis = (apoapsis_r + periapsis_r) / 2.0;
        let eccentricity = (apoapsis_r - periapsis_r) / (apoapsis_r + periapsis_r);

        egui::Panel::left("fido_telemetry_panel").show(ui, |ui| {
            ui.heading(egui::RichText::new("TELEMETRY").color(RETRO_RED));
            ui.separator();
            ui.label(format!("Apoapsis altitude: {:.0} m", apoapsis));
            ui.label(format!("Periapsis altitude: {:.0} m", periapsis));
            ui.label(format!("Semi major axis: {:.0} m", semi_major_axis));
            ui.label(format!("Eccentricity: {:.4}", eccentricity));
            ui.label(format!(
                "Argument of periapsis: {:.2} rad",
                argument_of_periapsis
            ));
            ui.label(format!("True anomaly: {:.2} rad", true_anomaly));
            ui.label(format!("Body radius: {:.0} m", body_radius));
            ui.label(format!("Inclination: {:.2} rad", inclination));
            ui.label(format!("Surface speed: {:.1} m/s", surface_speed));
            ui.label(format!("Orbital Speed: {:.1} m/s", orbital_speed));
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].heading(egui::RichText::new("TOP DOWN VIEW").color(RETRO_RED));
                draw_orbit_top_view(
                    &mut columns[0],
                    apoapsis,
                    periapsis,
                    argument_of_periapsis,
                    true_anomaly,
                    body_radius,
                    surface_speed,
                );

                columns[1].heading(egui::RichText::new("SIDE VIEW (INCLINATION)").color(RETRO_RED));
                draw_orbit_side_view(
                    &mut columns[1],
                    apoapsis,
                    periapsis,
                    argument_of_periapsis,
                    true_anomaly,
                    inclination,
                    body_radius,
                );
            });
        });
    }
}

fn draw_orbit_top_view(
    ui: &mut egui::Ui,
    apoapsis: f64,
    periapsis: f64,
    argument_of_periapsis: f64,
    true_anomaly: f64,
    body_radius: f64,
    speed: f64,
) {
    let painter = ui.painter();
    let screen = ui.max_rect();
    let center = screen.center();
    let apoapsis_r = apoapsis + body_radius;
    let periapsis_r = periapsis + body_radius;
    let semi_major_axis = (apoapsis_r + periapsis_r) / 2.0;
    let eccentricity = (apoapsis_r - periapsis_r) / (apoapsis_r + periapsis_r);
    let max_radius_px = (screen.width().min(screen.height()) / 2.0) as f64 * 0.9;
    let scale = max_radius_px / apoapsis_r;

    painter.circle_filled(center, (body_radius * scale) as f32, RETRO_BLUE);

    let steps = 180;
    let mut points: Vec<egui::Pos2> = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let theta = (i as f64 / steps as f64) * 2.0 * PI;
        points.push(orbit_point(
            theta,
            semi_major_axis,
            eccentricity,
            argument_of_periapsis,
            center,
            scale,
        ));
    }
    painter.add(egui::Shape::line(
        points,
        egui::Stroke::new(1.5, RETRO_AMBER),
    ));

    let ship_r =
        semi_major_axis * (1.0 - eccentricity.powi(2)) / (1.0 + eccentricity * true_anomaly.cos());
    let ship_angle = true_anomaly + argument_of_periapsis;
    let ship_pos = egui::pos2(
        center.x + (ship_r * scale * ship_angle.cos()) as f32,
        center.y - (ship_r * scale * ship_angle.sin()) as f32,
    );
    painter.circle_filled(ship_pos, 4.0, RETRO_RED);

    draw_velocity_vector(
        painter,
        ship_pos,
        true_anomaly,
        eccentricity,
        argument_of_periapsis,
        semi_major_axis,
        speed,
        RETRO_GREEN,
    );
}

fn draw_velocity_vector(
    painter: &egui::Painter,
    ship_pos: egui::Pos2,
    true_anomaly: f64,
    eccentricity: f64,
    argument_of_periapsis: f64,
    semi_major_axis: f64,
    speed: f64,
    color: egui::Color32,
) {
    let theta = true_anomaly;
    let denom = 1.0 + eccentricity * theta.cos();
    let r = semi_major_axis * (1.0 - eccentricity.powi(2)) / denom;
    let dr_dtheta =
        semi_major_axis * (1.0 - eccentricity.powi(2)) * eccentricity * theta.sin() / denom.powi(2);

    let angle = theta + argument_of_periapsis;
    let dx = dr_dtheta * angle.cos() - r * angle.sin();
    let dy = dr_dtheta * angle.sin() + r * angle.cos();

    let mag = (dx * dx + dy * dy).sqrt();
    if mag < 1e-9 {
        return;
    }

    let dir = egui::vec2((dx / mag) as f32, -(dy / mag) as f32);
    let arrow_len = (speed * 0.03).clamp(20.0, 140.0) as f32;
    let tip = ship_pos + dir * arrow_len;

    painter.line_segment([ship_pos, tip], egui::Stroke::new(2.0, color));

    let head_len = 8.0;
    let head_angle: f32 = 0.5;
    let (s, c) = head_angle.sin_cos();
    let back = -dir;
    let left = egui::vec2(back.x * c - back.y * s, back.x * s + back.y * c) * head_len;
    let right = egui::vec2(back.x * c + back.y * s, -back.x * s + back.y * c) * head_len;

    painter.line_segment([tip, tip + left], egui::Stroke::new(2.0, color));
    painter.line_segment([tip, tip + right], egui::Stroke::new(2.0, color));
}

fn draw_orbit_side_view(
    ui: &mut egui::Ui,
    apoapsis: f64,
    periapsis: f64,
    argument_of_periapsis: f64,
    true_anomaly: f64,
    inclination: f64,
    body_radius: f64,
) {
    let painter = ui.painter();
    let screen = ui.max_rect();
    let center = screen.center();

    let apoapsis_r = apoapsis + body_radius;
    let periapsis_r = periapsis + body_radius;
    let semi_major_axis = (apoapsis_r + periapsis_r) / 2.0;
    let eccentricity = (apoapsis_r - periapsis_r) / (apoapsis_r + periapsis_r);

    let max_radius_px = (screen.width().min(screen.height()) / 2.0) as f64 * 0.9;
    let scale = max_radius_px / apoapsis_r;

    let ref_line_half_width = max_radius_px as f32;
    painter.line_segment(
        [
            egui::pos2(center.x - ref_line_half_width, center.y),
            egui::pos2(center.x + ref_line_half_width, center.y),
        ],
        egui::Stroke::new(1.0, RETRO_GRID),
    );

    painter.circle_filled(center, (body_radius * scale) as f32, RETRO_BLUE);

    let steps = 180;
    let mut points: Vec<egui::Pos2> = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let theta = (i as f64 / steps as f64) * 2.0 * PI;
        points.push(orbit_side_point(
            theta,
            semi_major_axis,
            eccentricity,
            argument_of_periapsis,
            inclination,
            center,
            scale,
        ));
    }
    painter.add(egui::Shape::line(
        points,
        egui::Stroke::new(1.5, RETRO_AMBER),
    ));

    let ship_pos = orbit_side_point(
        true_anomaly,
        semi_major_axis,
        eccentricity,
        argument_of_periapsis,
        inclination,
        center,
        scale,
    );
    painter.circle_filled(ship_pos, 4.0, RETRO_RED);
}

fn orbit_side_point(
    theta: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    argument_of_periapsis: f64,
    inclination: f64,
    center: egui::Pos2,
    scale: f64,
) -> egui::Pos2 {
    let r = semi_major_axis * (1.0 - eccentricity.powi(2)) / (1.0 + eccentricity * theta.cos());
    let angle = theta + argument_of_periapsis;
    let x_orb = r * angle.cos();
    let y_orb = r * angle.sin();
    let z = y_orb * inclination.sin();

    egui::pos2(
        center.x + (x_orb * scale) as f32,
        center.y - (z * scale) as f32,
    )
}

fn orbit_point(
    theta: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    argument_of_periapsis: f64,
    center: egui::Pos2,
    scale: f64,
) -> egui::Pos2 {
    let r = semi_major_axis * (1.0 - eccentricity.powi(2)) / (1.0 + eccentricity * theta.cos());
    let angle = theta + argument_of_periapsis;
    egui::pos2(
        center.x + (r * scale * angle.cos()) as f32,
        center.y - (r * scale * angle.sin()) as f32,
    )
}
