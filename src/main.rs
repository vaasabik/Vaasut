#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Rust Engine",
        native_options,
        Box::new(|cc| Ok(Box::new(engine_demo::EngineApp::new(cc)))),
    )
}
