use crate::html::HtmlDom;
use wasm_bindgen::JsValue;

pub struct AppState {
    pub html: HtmlDom,
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        Self { html }
    }

    pub fn init_audio(&mut self) -> Result<(), JsValue> {
        self.html.init_audio()
    }
}
