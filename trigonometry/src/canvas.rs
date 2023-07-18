use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement};
use crate::app_state::AppState;
use crate::draw::DrawWithState;

#[derive(Clone)]
pub struct Canvas {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub offset: [i32; 2],
}

impl Canvas {
    pub fn create(document: &Document, id: &str) -> Result<Self, JsValue> {
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let offset = [canvas.width() as i32 / 2, canvas.height() as i32 / 2];
        let _ = context.translate(offset[0].into(), offset[1].into());

        Ok(Self {
            canvas,
            context,
            offset,
        })
    }
}

// impl DrawWithState for Canvas {
//     fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
//
//     }
// }