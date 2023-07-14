use commons::utils::OkExt;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, Element};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub container: Element,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let container = document.get_element_by_id("container").unwrap();

        Self {
            document,
            container,
        }
        .ok()
    }
}
