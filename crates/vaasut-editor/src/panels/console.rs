use eframe::egui;

/// Консоль
pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("console").show(ctx, |ui| {
        ui.heading("Console");
        ui.label("✅ Engine initialized");
    });
}
