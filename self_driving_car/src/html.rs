use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};
use crate::car::Car;

pub struct HtmlDom {
    pub window: Window,
    pub document: Document,
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub car: Car,
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

        let car = Car::create(context.clone(), 0.0, 0.0, 100.0, 100.0);

        Self {
            window,
            document,
            canvas,
            context,
            car,
        }
        .ok()
    }
}
