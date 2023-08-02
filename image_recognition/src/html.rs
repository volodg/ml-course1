use wasm_bindgen::JsValue;
use web_sys::{Document, window};
use commons::utils::OkExt;

pub struct HtmlDom {
    pub document: Document,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        Self {
            document,
        }
        .ok()
    }
}
