use commons::utils::OkExt;
use wasm_bindgen::JsValue;
use web_sys::{window, Document};

pub struct HtmlDom {
    pub document: Document,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        Self { document }.ok()
    }
}
