use eframe::egui;

/// Центральный вьюпорт
pub fn show(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Viewport");
        ui.label("3D/2D scene will be rendered here");
    });
}
