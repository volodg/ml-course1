use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub context: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
    pub offset: [i32; 2],
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("myCanvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let offset = [canvas.width() as i32 / 2, canvas.height() as i32 / 2];

        let _ = context.translate(offset[0].into(), offset[1].into());

        Self {
            document,
            context,
            canvas,
            offset,
        }
        .ok()
    }
}
