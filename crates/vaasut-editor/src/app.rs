use eframe::egui;

/// Главный редактор
pub struct VaasutEditor {
    time: f32,
    show_hierarchy: bool,
    show_inspector: bool,
    show_console: bool,
}

impl VaasutEditor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            time: 0.0,
            show_hierarchy: true,
            show_inspector: true,
            show_console: true,
        }
    }
}

impl eframe::App for VaasutEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016;

        // Определяем ориентацию экрана
        let screen_size = ctx.input(|i| i.viewport_rect).size();
        let is_portrait = screen_size.x < screen_size.y;

        // 1. Top панель (всегда первая)
        super::panels::toolbar::show(ctx);

        // 2. Bottom панель
        if self.show_console {
            super::panels::console::show(ctx, is_portrait);
        }

        // 3. Side панели (только в альбомной ориентации)
        if !is_portrait {
            if self.show_hierarchy {
                super::panels::hierarchy::show(ctx);
            }
            if self.show_inspector {
                super::panels::inspector::show(ctx);
            }
        }

        // 4. Central панель ВСЕГДА последняя
        super::panels::viewport::show(ctx, is_portrait);

        ctx.request_repaint();
    }
}
