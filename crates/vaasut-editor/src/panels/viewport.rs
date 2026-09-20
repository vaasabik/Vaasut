use eframe::egui;

/// Центральный вьюпорт
pub fn show(ctx: &egui::Context, is_portrait: bool) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let available = ui.available_size();
        
        if is_portrait {
            // Портретная ориентация - минимум информации
            ui.vertical_centered(|ui| {
                ui.heading("Viewport");
                ui.label(format!("Size: {:.0}x{:.0}", available.x, available.y));
                ui.separator();
                ui.label("3D/2D scene will be rendered here");
                ui.label("");
                ui.label("📱 Portrait mode - limited space");
            });
        } else {
            // Альбомная ориентация
            ui.vertical_centered(|ui| {
                ui.heading("Viewport");
                ui.label(format!("Size: {:.0}x{:.0}", available.x, available.y));
                ui.separator();
                ui.label("3D/2D scene will be rendered here");
            });
        }
    });
}
