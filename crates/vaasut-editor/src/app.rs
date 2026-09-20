use eframe::egui;

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
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vaasut Editor - Modular Architecture");
            ui.label("All systems are now separate crates!");
        });
        
        ctx.request_repaint();
    }
}
