mod connection;
mod flight_state;
mod gui;
mod telemetry;

use crate::connection::init_connection;
use crate::gui::mocr_app::MocrApp;
use crate::telemetry::get_telemetry;
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
            crate::gui::theme::apply(&cc.egui_ctx);
            Ok(Box::new(MocrApp::new(telemetry)))
        }),
    )
    .unwrap();
}
