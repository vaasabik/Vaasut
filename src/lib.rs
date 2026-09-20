use eframe::egui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            "vaasut_canvas",
            web_options,
            Box::new(|cc| Ok(Box::new(VaasutEditor::new(cc)))),
        )
        .await
        .expect("Failed to start Vaasut");
}

// ========================
//  Состояние сцены
// ========================
struct SceneObject {
    name: String,
    icon: String,
    position: [f32; 3],
    selected: bool,
}

// ========================
//  Главный редактор
// ========================
pub struct VaasutEditor {
    time: f32,
    objects: Vec<SceneObject>,
    show_about: bool,
}

impl VaasutEditor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            time: 0.0,
            show_about: false,
            objects: vec![
                SceneObject {
                    name: "Main Camera".into(),
                    icon: "🎥".into(),
                    position: [0.0, 5.0, -10.0],
                    selected: false,
                },
                SceneObject {
                    name: "Directional Light".into(),
                    icon: "💡".into(),
                    position: [0.0, 10.0, 0.0],
                    selected: false,
                },
                SceneObject {
                    name: "Cube_Player".into(),
                    icon: "🟦".into(),
                    position: [0.0, 1.0, 0.0],
                    selected: true,
                },
                SceneObject {
                    name: "Plane_Ground".into(),
                    icon: "🟩".into(),
                    position: [0.0, 0.0, 0.0],
                    selected: false,
                },
                SceneObject {
                    name: "Sphere_Enemy".into(),
                    icon: "🔴".into(),
                    position: [3.0, 1.0, 2.0],
                    selected: false,
                },
            ],
        }
    }
}

impl eframe::App for VaasutEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016;

        // ===== ВЕРХНЕЕ МЕНЮ =====
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Scene").clicked() { ui.close_menu(); }
                    if ui.button("Open Scene...").clicked() { ui.close_menu(); }
                    if ui.button("Save Scene").clicked() { ui.close_menu(); }
                    ui.separator();
                    if ui.button("Build & Run").clicked() { ui.close_menu(); }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo  Ctrl+Z").clicked() { ui.close_menu(); }
                    if ui.button("Redo  Ctrl+Y").clicked() { ui.close_menu(); }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Reset Layout").clicked() { ui.close_menu(); }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About Vaasut").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("⚙ Vaasut v0.1.0");
                });
            });
        });

        // ===== НИЖНЯЯ ПАНЕЛЬ (консоль) =====
        egui::TopBottomPanel::bottom("console").show(ctx, |ui| {
            ui.heading("Console");
            ui.label("✅ [Vaasut] Engine initialized");
            ui.label("✅ [Vaasut] Scene loaded: default_scene.vaasut");
            ui.label(format!("🔄 [Vaasut] Frame time: {:.1}ms", 16.0));
        });

        // ===== ЛЕВАЯ ПАНЕЛЬ (иерархия) =====
        egui::SidePanel::left("hierarchy")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("📁 Scene Hierarchy");
                ui.separator();

                for obj in &mut self.objects {
                    let label = format!("{} {}", obj.icon, obj.name);
                    let response = ui.selectable_label(obj.selected, &label);
                    if response.clicked() {
                        for o in &mut self.objects {
                            o.selected = false;
                        }
                        obj.selected = true;
                    }
                }

                ui.separator();
                if ui.button("➕ Add Object").clicked() {
                    self.objects.push(SceneObject {
                        name: format!("Object_{}", self.objects.len()),
                        icon: "📦".into(),
                        position: [0.0, 0.0, 0.0],
                        selected: false,
                    });
                }
            });

        // ===== ПРАВАЯ ПАНЕЛЬ (инспектор) =====
        egui::SidePanel::right("inspector")
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("🔍 Inspector");
                ui.separator();

                if let Some(selected) = self.objects.iter_mut().find(|o| o.selected) {
                    ui.label(format!("{} {}", selected.icon, selected.name));
                    ui.separator();

                    ui.label("Transform:");
                    ui.horizontal(|ui| {
                        ui.label("X");
                        ui.add(egui::DragValue::new(&mut selected.position[0]).speed(0.1));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Y");
                        ui.add(egui::DragValue::new(&mut selected.position[1]).speed(0.1));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Z");
                        ui.add(egui::DragValue::new(&mut selected.position[2]).speed(0.1));
                    });

                    ui.separator();
                    ui.label("Components:");
                    ui.checkbox(&mut true, "Mesh Renderer");
                    ui.checkbox(&mut true, "Box Collider");
                    ui.checkbox(&mut false, "Rigidbody");
                } else {
                    ui.label("No object selected");
                }
            });

        // ===== ЦЕНТРАЛЬНЫЙ ВЬЮПОРТ =====
        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            let painter = ui.painter_at(rect);

            // Фон
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(26, 26, 46));

            // Сетка (пол)
            let grid_color = egui::Color32::from_rgb(50, 50, 80);
            let cols = 20;
            let rows = 14;
            for i in 0..=cols {
                let x = rect.left() + (rect.width() / cols as f32) * i as f32;
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    (1.0, grid_color),
                );
            }
            for i in 0..=rows {
                let y = rect.top() + (rect.height() / rows as f32) * i as f32;
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    (1.0, grid_color),
                );
            }

            // Оси координат в центре
            let center = rect.center();
            let axis_len = 60.0;
            // X — красная
            painter.line_segment(
                [center, center + egui::vec2(axis_len, 0.0)],
                (2.0, egui::Color32::RED),
            );
            // Y — зелёная
            painter.line_segment(
                [center, center + egui::vec2(0.0, -axis_len)],
                (2.0, egui::Color32::GREEN),
            );

            // Анимированный "игрок" (куб)
            let player_x = center.x + (self.time * 0.5).sin() * 100.0;
            let player_y = center.y + (self.time * 0.8).cos() * 40.0;
            let player_size = 20.0;
            let player_rect = egui::Rect::from_center_size(
                egui::pos2(player_x, player_y),
                egui::vec2(player_size, player_size),
            );
            painter.rect_filled(player_rect, 3.0, egui::Color32::from_rgb(0, 150, 255));
            
            // ИСПРАВЛЕНИЕ: явное указание типа f32 для ширины линии
            painter.rect_stroke(
                player_rect,
                3.0,
                egui::Stroke::new(2.0_f32, egui::Color32::WHITE),
            );

            // "Враг" (круг)
            let enemy_x = center.x + 120.0;
            let enemy_y = center.y + (self.time * 1.2).sin() * 60.0;
            painter.circle_filled(
                egui::pos2(enemy_x, enemy_y),
                15.0,
                egui::Color32::from_rgb(233, 69, 96),
            );

            // Подпись
            painter.text(
                egui::pos2(rect.left() + 10.0, rect.top() + 15.0),
                egui::Align2::LEFT_TOP,
                "Vaasut Viewport (2D preview — wgpu 3D coming soon)",
                egui::FontId::proportional(12.0),
                egui::Color32::from_rgb(150, 150, 170),
            );
        });

        // ===== ОКНО "О ПРОГРАММЕ" =====
        if self.show_about {
            egui::Window::new("About Vaasut")
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.heading("⚙ Vaasut Engine");
                    ui.label("Version 0.1.0");
                    ui.label("A cross-platform game engine written in Rust.");
                    ui.separator();
                    ui.label("Stack: wgpu + egui + winit");
                    ui.label("Targets: Desktop / Web / Mobile");
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }

        ctx.request_repaint();
    }
}
