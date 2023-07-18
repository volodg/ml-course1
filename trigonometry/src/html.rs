use crate::chart_canvas::ChartCanvas;
use commons::utils::OkExt;
use wasm_bindgen::JsValue;
use web_sys::{window, Document};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub canvas: ChartCanvas,
    pub chart_canvas: ChartCanvas,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = ChartCanvas::create(&document, "myCanvas")?;
        let chart_canvas = ChartCanvas::create(&document, "chartCanvas")?;

        Self {
            document,
            canvas,
            chart_canvas,
        }
        .ok()
    }
}
