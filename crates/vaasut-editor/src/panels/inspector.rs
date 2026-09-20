use eframe::egui;

/// Панель инспектора свойств
pub fn show(ctx: &egui::Context) {
    egui::SidePanel::right("inspector")
        .default_width(220.0)
        .width_range(150.0..=400.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();
            
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.label("No object selected");
                    ui.separator();
                    ui.label("Transform:");
                    ui.label("  Position: 0, 0, 0");
                    ui.label("  Rotation: 0, 0, 0");
                    ui.label("  Scale: 1, 1, 1");
                });
        });
}
