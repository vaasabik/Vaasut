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

        // 1. СНАЧАЛА Top/Bottom панели
        super::panels::toolbar::show(ctx);      // Top panel
        super::panels::console::show(ctx);      // Bottom panel

        // 2. ПОТОМ Side панели
        super::panels::hierarchy::show(ctx);   // Left panel
        super::panels::inspector::show(ctx);   // Right panel

        // 3. В САМОМ КОНЦЕ Central панель
        super::panels::viewport::show(ctx);    // Central panel (ВСЕГДА ПОСЛЕДНИМ!)

        ctx.request_repaint();
    }
}