use eframe::egui;

/// Консоль
pub fn show(ctx: &egui::Context, is_portrait: bool) {
    let height = if is_portrait { 80.0 } else { 120.0 };
    
    egui::TopBottomPanel::bottom("console")
        .min_height(60.0)
        .default_height(height)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Console");
            });
            ui.separator();
            
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.label("✅ Engine initialized");
                    ui.label("📱 Mobile mode detected");
                });
        });
}
