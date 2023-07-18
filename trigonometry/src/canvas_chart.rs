use crate::app_state::AppState;
use crate::canvas::Canvas;
use crate::draw::DrawWithState;
use wasm_bindgen::JsValue;
use web_sys::Document;

#[derive(Clone)]
pub struct CanvasChart {
    pub canvas: Canvas,
}

impl CanvasChart {
    pub fn create(document: &Document, id: &str) -> Result<Self, JsValue> {
        let canvas = Canvas::create(document, id)?;

        Ok(Self { canvas })
    }
}

impl DrawWithState for CanvasChart {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.canvas.draw(app_state)
    }
}
