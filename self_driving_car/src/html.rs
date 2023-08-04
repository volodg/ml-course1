use crate::car::Car;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

pub struct HtmlDom {
    pub window: Window,
    pub document: Document,
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub car: Rc<RefCell<Car>>,
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

        let car = Car::create(context.clone(), 100.0, 100.0, 30.0, 50.0)?;

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
