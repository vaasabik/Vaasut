#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    // Получаем canvas элемент из DOM
    let canvas = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document")
        .get_element_by_id("vaasut_canvas")
        .expect("No canvas element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Element is not a canvas");

    let web_options = eframe::WebOptions::default();
    
    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|cc| Ok(Box::new(vaasut_editor::VaasutEditor::new(cc)))),
        )
        .await
        .expect("Failed to start Vaasut");
}