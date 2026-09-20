use eframe::egui;

/// Главный редактор
pub struct VaasutEditor {
    time: f32,
}

impl VaasutEditor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { time: 0.0 }
    }
}

impl eframe::App for VaasutEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016;
        
        // Рисуем панели
        super::panels::toolbar::show(ctx);
        super::panels::hierarchy::show(ctx);
        super::panels::inspector::show(ctx);
        super::panels::viewport::show(ctx);
        super::panels::console::show(ctx);
        
        ctx.request_repaint();
    }
}
