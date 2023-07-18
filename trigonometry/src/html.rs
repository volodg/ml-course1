use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub offset: [i32; 2],
    pub chart_canvas: HtmlCanvasElement,
    pub chart_context: CanvasRenderingContext2d,
    pub chart_offset: [i32; 2],
}

impl HtmlDom {
    fn get_canvas_and_context(document: &Document, id: &str) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok((canvas, context))
    }

    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let (canvas, context) = Self::get_canvas_and_context(&document, "myCanvas")?;
        let (chart_canvas, chart_context) = Self::get_canvas_and_context(&document, "chartCanvas")?;

        let offset = [canvas.width() as i32 / 2, canvas.height() as i32 / 2];
        let _ = context.translate(offset[0].into(), offset[1].into());

        let chart_offset = [chart_canvas.width() as i32 / 2, chart_canvas.height() as i32 / 2];
        let _ = chart_context.translate(chart_offset[0].into(), chart_offset[1].into());

        Self {
            document,
            canvas,
            context,
            offset,
            chart_canvas,
            chart_context,
            chart_offset,
        }
        .ok()
    }
}
