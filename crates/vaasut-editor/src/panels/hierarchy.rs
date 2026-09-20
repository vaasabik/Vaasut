use eframe::egui;

/// Панель иерархии сцены
pub fn show(ctx: &egui::Context) {
    egui::SidePanel::left("hierarchy")
        .resizable(true)
        .default_width(200.0)
        .min_width(150.0)
        .max_width(400.0)
        .show(ctx, |ui| {
            ui.heading("Scene Hierarchy");
            ui.label("📦 Main Camera");
            ui.label("💡 Directional Light");
        });
}