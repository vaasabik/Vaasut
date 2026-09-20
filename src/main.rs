//! Vaasut Engine — Desktop entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Vaasut Engine"),
        ..Default::default()
    };

    eframe::run_native(
        "Vaasut Engine",
        native_options,
        Box::new(|cc| Ok(Box::new(vaasut::VaasutApp::new(cc)))),
    )
}
