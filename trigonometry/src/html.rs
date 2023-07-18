use crate::canvas_chart::CanvasChart;
use crate::canvas_graphic::CanvasGraphic;
use commons::utils::OkExt;
use wasm_bindgen::JsValue;
use web_sys::{window, Document};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub canvas: CanvasGraphic,
    pub chart_canvas: CanvasChart,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = CanvasGraphic::create(&document, "myCanvas")?;
        let chart_canvas = CanvasChart::create(&document, "chartCanvas")?;

        Self {
            document,
            canvas,
            chart_canvas,
        }
        .ok()
    }
}
