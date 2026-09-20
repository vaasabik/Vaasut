use eframe::egui;

/// Панель инструментов
pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar")
        .min_height(32.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("⚙ Vaasut");
                ui.separator();
                
                if ui.button("▶ Play").clicked() {
                    log::info!("Play clicked");
                }
                if ui.button("⏸").clicked() {
                    log::info!("Pause clicked");
                }
                if ui.button("⏹").clicked() {
                    log::info!("Stop clicked");
                }
                
                ui.separator();
                ui.label("Mode: Edit");
            });
        });
}
