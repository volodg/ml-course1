use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub context: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

impl HtmlDom {
    #[allow(dead_code)]
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("myCanvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Self {
            document,
            context,
            canvas,
        }
        .ok()
    }
}
