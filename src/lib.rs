//! Vaasut Engine — Core library
//!
//! This crate contains the engine core that works on both
//! desktop (native) and web (WASM) targets.

use eframe::egui;

// ─── WASM entry point ────────────────────────────────────────
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    // Redirect log output to browser console
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            "vaasut_canvas",
            web_options,
            Box::new(|cc| Ok(Box::new(VaasutApp::new(cc)))),
        )
        .await
        .expect("Failed to start Vaasut");
}

// ─── Engine State ────────────────────────────────────────────
pub struct VaasutApp {
    // Scene
    rotation: f32,
    cube_position: [f32; 3],
    cube_scale: f32,

    // Editor UI state
    show_grid: bool,
    selected_object: Option<String>,
}

impl VaasutApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure egui style (dark theme, Rust-orange accent)
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(249, 115, 22);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(234, 88, 12);
        cc.egui_ctx.set_style(style);

        Self {
            rotation: 0.0,
            cube_position: [0.0, 1.0, 0.0],
            cube_scale: 1.0,
            show_grid: true,
            selected_object: Some("Cube_01".to_string()),
        }
    }
}

// ─── Main render loop ────────────────────────────────────────
impl eframe::App for VaasutApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.rotation += 0.01;

        // ── Top Menu Bar ──
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚙ Vaasut").color(egui::Color32::from_rgb(249, 115, 22)).strong());
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui.button("New Scene").clicked() { ui.close_menu(); }
                    if ui.button("Save Scene").clicked() { ui.close_menu(); }
                    if ui.button("Open Scene").clicked() { ui.close_menu(); }
                    ui.separator();
                    if ui.button("Build & Run").clicked() { ui.close_menu(); }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo  Ctrl+Z").clicked() { ui.close_menu(); }
                    if ui.button("Redo  Ctrl+Y").clicked() { ui.close_menu(); }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_grid, "Show Grid");
                });
            });
        });

        // ── Bottom Status Bar ──
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("FPS: {:.0}", ctx.input(|i| 1.0 / i.stable_dt)));
                    ui.separator();
                    ui.label("Vaasut v0.1.0");
                });
            });
        });

        // ── Left Panel: Scene Hierarchy ──
        egui::SidePanel::left("hierarchy")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("📂 Hierarchy");
                ui.separator();

                let objects = [
                    ("📷", "Main Camera"),
                    ("💡", "Directional Light"),
                    ("🟦", "Cube_01"),
                    ("🟩", "Plane_Ground"),
                ];

                for (icon, name) in objects {
                    let is_selected = self.selected_object.as_deref() == Some(name);
                    let response = ui.selectable_label(is_selected, format!("{} {}", icon, name));
                    if response.clicked() {
                        self.selected_object = Some(name.to_string());
                    }
                }
            });

        // ── Right Panel: Inspector ──
        egui::SidePanel::right("inspector")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("🔍 Inspector");
                ui.separator();

                if let Some(ref name) = self.selected_object {
                    ui.label(egui::RichText::new(name).strong().size(16.0));
                    ui.separator();

                    ui.collapsing("Transform", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Position");
                            ui.add(egui::DragValue::new(&mut self.cube_position[0]).prefix("X: ").speed(0.1));
                            ui.add(egui::DragValue::new(&mut self.cube_position[1]).prefix("Y: ").speed(0.1));
                            ui.add(egui::DragValue::new(&mut self.cube_position[2]).prefix("Z: ").speed(0.1));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Rotation");
                            ui.add(egui::DragValue::new(&mut self.rotation).prefix("Y: ").speed(0.01));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Scale");
                            ui.add(egui::Slider::new(&mut self.cube_scale, 0.1..=5.0));
                        });
                    });

                    ui.collapsing("Mesh Renderer", |ui| {
                        let mut cast_shadows = true;
                        ui.checkbox(&mut cast_shadows, "Cast Shadows");
                        let mut receive_shadows = true;
                        ui.checkbox(&mut receive_shadows, "Receive Shadows");
                    });
                } else {
                    ui.label("No object selected");
                }
            });

        // ── Center: 3D Viewport ──
        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );
            let painter = ui.painter_at(rect);

            // Background
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(26, 26, 46));

            // Grid
            if self.show_grid {
                let grid_color = egui::Color32::from_rgb(50, 50, 70);
                let step = 40.0;
                let mut x = rect.left();
                while x <= rect.right() {
                    painter.line_segment(
                        [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                        (1.0, grid_color),
                    );
                    x += step;
                }
                let mut y = rect.top();
                while y <= rect.bottom() {
                    painter.line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        (1.0, grid_color),
                    );
                    y += step;
                }
            }

            // Draw "3D object" (rotating polygon representing a cube)
            let center = rect.center();
            let size = 40.0 * self.cube_scale;
            let cos_r = self.rotation.cos();
            let sin_r = self.rotation.sin();

            let points = vec![
                center + egui::vec2(size * cos_r, size * sin_r),
                center + egui::vec2(-size * sin_r, size * cos_r),
                center + egui::vec2(-size * cos_r, -size * sin_r),
                center + egui::vec2(size * sin_r, -size * cos_r),
            ];

            painter.add(egui::Shape::convex_polygon(
                points,
                egui::Color32::from_rgb(0, 150, 255),
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            ));

            // Gizmo hint
            painter.text(
                egui::pos2(rect.left() + 10.0, rect.bottom() - 10.0),
                egui::Align2::LEFT_BOTTOM,
                "Viewport — wgpu 3D rendering coming soon",
                egui::FontId::proportional(12.0),
                egui::Color32::from_rgb(100, 100, 120),
            );
        });

        // Keep animating
        ctx.request_repaint();
    }
}
