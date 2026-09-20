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
            Box::new(|cc| Ok(Box::new(vaasut_editor::VaasutEditor::new(cc)))),
        )
        .await
        .expect("Failed to start Vaasut");
}
