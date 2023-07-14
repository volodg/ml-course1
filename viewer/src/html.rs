use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, Element, HtmlCanvasElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub container: Element,
    pub canvas: HtmlCanvasElement,
}

impl HtmlDom {
    #[allow(dead_code)]
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let container = document.get_element_by_id("container").unwrap();

        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        Self {
            document,
            container,
            canvas
        }
        .ok()
    }
}
