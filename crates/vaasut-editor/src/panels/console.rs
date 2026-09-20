use eframe::egui;

/// Консоль
pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("console")
        .resizable(true)
        .default_height(150.0)
        .min_height(100.0)
        .max_height(300.0)
        .show(ctx, |ui| {
            ui.heading("Console");
            ui.label("✅ Engine initialized");
        });
}