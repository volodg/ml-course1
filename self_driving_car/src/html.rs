use crate::car::Car;
use crate::road::Road;
use commons::geometry::{Point2D, Point2DView};
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
    pub road: Road,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let canvas = document.get_element_by_id("myCanvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(200);

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let road = Road::create(
            context.clone(),
            canvas.width() as f64 / 2.0,
            canvas.width() as f64 * 0.9,
        );

        let car = Car::create(
            context.clone(),
            Point2D::create(road.get_lane_center(1), 100.0),
            30.0,
            50.0,
        )?;

        Self {
            window,
            document,
            canvas,
            context,
            car,
            road,
        }
        .ok()
    }
}
