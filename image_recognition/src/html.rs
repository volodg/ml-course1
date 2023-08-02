use commons::utils::OkExt;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlCanvasElement};

pub struct HtmlDom {
    pub document: Document,
    pub canvas: HtmlCanvasElement,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        Self { document, canvas }.ok()
    }
}
