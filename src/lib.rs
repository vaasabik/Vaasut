use eframe::egui;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            "the_canvas_id",
            web_options,
            Box::new(|cc| Ok(Box::new(EngineApp::new(cc)))),
        )
        .await
        .expect("failed to start eframe");
}

pub struct EngineApp {
    rotation: f32,
}

impl EngineApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { rotation: 0.0 }
    }
}

impl eframe::App for EngineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.rotation += 0.01;

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Scene").clicked() {}
                    if ui.button("Exit").clicked() {}
                });
            });
        });

        egui::SidePanel::left("scene_hierarchy").show(ctx, |ui| {
            ui.heading("Scene Hierarchy");
            ui.label("📦 Main Camera");
            ui.label("💡 Directional Light");
            ui.label("📦 Cube_01");
        });

        egui::SidePanel::right("inspector").show(ctx, |ui| {
            ui.heading("Inspector");
            ui.label("Transform:");
            ui.add(egui::DragValue::new(&mut self.rotation).prefix("Rot Y: "));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            let painter = ui.painter_at(rect);
            
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 30, 35));

            let grid_color = egui::Color32::from_rgb(60, 60, 70);
            for i in 0..=10 {
                let x = rect.left() + (rect.width() / 10.0) * i as f32;
                painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], (1.0, grid_color));
                let y = rect.top() + (rect.height() / 10.0) * i as f32;
                painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], (1.0, grid_color));
            }

            let center = rect.center();
            let size = 50.0;
            let points = [
                center + egui::vec2(size * self.rotation.cos(), size * self.rotation.sin()),
                center + egui::vec2(-size * self.rotation.sin(), size * self.rotation.cos()),
                center + egui::vec2(-size * self.rotation.cos(), -size * self.rotation.sin()),
                center + egui::vec2(size * self.rotation.sin(), -size * self.rotation.cos()),
            ];
            painter.add(egui::Shape::convex_polygon(points, egui::Color32::from_rgb(0, 150, 255), egui::Stroke::new(2.0, egui::Color32::WHITE)));
            
            ui.label("Viewport (Drag to rotate)");
        });

        ctx.request_repaint();
    }
}
