fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vaasut Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(vaasut_editor::VaasutEditor::new(cc)))),
    )
}
