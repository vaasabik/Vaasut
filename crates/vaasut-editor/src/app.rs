use eframe::egui;
use vaasut_renderer::RenderMode;

pub struct VaasutEditor {
    time: f32,
    render_mode: RenderMode,
}

impl VaasutEditor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { 
            time: 0.0,
            render_mode: RenderMode::D3, // По умолчанию 3D режим
        }
    }
}

impl eframe::App for VaasutEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016;
        
        // Верхняя панель с переключателем режимов
        egui::TopBottomPanel::top("mode_switcher").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Vaasut Editor");
                ui.separator();
                
                ui.label("Режим:");
                ui.selectable_value(&mut self.render_mode, RenderMode::D2, "2D");
                ui.selectable_value(&mut self.render_mode, RenderMode::D3, "3D");
                ui.selectable_value(&mut self.render_mode, RenderMode::Mixed, "Mixed");
            });
        });
        
        // Центральный вьюпорт
        egui::CentralPanel::default().show(ctx, |ui| {
            let text = match self.render_mode {
                RenderMode::D2 => "2D Viewport (Sprite Editor)",
                RenderMode::D3 => "3D Viewport (Model Editor)",
                RenderMode::Mixed => "Mixed Viewport (2D UI + 3D World)",
            };
            
            ui.heading(text);
            ui.label("Здесь будет рендериться сцена");
        });
        
        ctx.request_repaint();
    }
}
