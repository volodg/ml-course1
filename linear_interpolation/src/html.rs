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
    pub fn create() -> Result<Self, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("myCanvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width: u32 = window.inner_width().expect("").as_f64().unwrap() as u32;
        canvas.set_width(width);

        let height: u32 = window.inner_height().expect("").as_f64().unwrap() as u32;
        canvas.set_height(height);

        Self {
            document,
            context,
            canvas,
        }
        .ok()
    }
}
