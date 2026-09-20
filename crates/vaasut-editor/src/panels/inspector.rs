use eframe::egui;

/// Панель инспектора свойств
pub fn show(ctx: &egui::Context) {
    egui::SidePanel::right("inspector")
        .resizable(true)
        .default_width(300.0)
        .min_width(200.0)
        .max_width(500.0)
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.label("No object selected");
        });
}