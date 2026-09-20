use eframe::egui;

/// Панель иерархии сцены
pub fn show(ctx: &egui::Context) {
    egui::SidePanel::left("hierarchy").show(ctx, |ui| {
        ui.heading("Scene Hierarchy");
        ui.label("📦 Main Camera");
        ui.label("💡 Directional Light");
    });
}
