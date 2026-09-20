use eframe::egui;

/// Панель инспектора свойств
pub fn show(ctx: &egui::Context) {
    egui::SidePanel::right("inspector").show(ctx, |ui| {
        ui.heading("Inspector");
        ui.label("No object selected");
    });
}
