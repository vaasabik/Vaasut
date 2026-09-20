use eframe::egui;

/// Панель иерархии сцены
pub fn show(ctx: &egui::Context) {
    egui::SidePanel::left("hierarchy")
        .default_width(180.0)
        .width_range(120.0..=300.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Hierarchy");
            ui.separator();
            
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.label("📦 Main Camera");
                    ui.label("💡 Directional Light");
                    ui.label("🎮 Player");
                });
        });
}
