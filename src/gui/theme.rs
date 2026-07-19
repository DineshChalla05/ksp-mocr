use eframe::egui;

pub const RETRO_BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 10);
pub const RETRO_AMBER: egui::Color32 = egui::Color32::from_rgb(255, 176, 0);
pub const RETRO_RED: egui::Color32 = egui::Color32::from_rgb(252, 61, 33);
pub const RETRO_BLUE: egui::Color32 = egui::Color32::from_rgb(11, 61, 145);
pub const RETRO_GRID: egui::Color32 = egui::Color32::from_rgb(60, 60, 60);
pub const RETRO_GREEN: egui::Color32 = egui::Color32::from_rgb(51, 255, 51);

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "nasalization".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Nasalization-Rg.otf"
        ))),
    );

    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "nasalization".to_owned());
    ctx.set_fonts(fonts);
}

fn apply_visuals(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.set_visuals(egui::Visuals {
        panel_fill: RETRO_BG,
        window_fill: RETRO_BG,
        override_text_color: Some(RETRO_AMBER),
        hyperlink_color: RETRO_RED,
        selection: egui::style::Selection {
            bg_fill: RETRO_BLUE,
            stroke: egui::Stroke::new(1.0, RETRO_AMBER),
        },
        ..egui::Visuals::dark()
    });
}

pub fn apply(ctx: &egui::Context) {
    install_fonts(ctx);
    apply_visuals(ctx);
}