use eframe::egui;

/// Панель инструментов
pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("⚙ Vaasut Editor");
            ui.separator();
            ui.button("▶ Play");
            ui.button("⏸ Pause");
            ui.button("⏹ Stop");
        });
    });
}
