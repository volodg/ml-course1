use crate::car::{Car, ControlType};
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
    pub car_canvas: HtmlCanvasElement,
    pub car_context: CanvasRenderingContext2d,
    pub network_canvas: HtmlCanvasElement,
    pub network_context: CanvasRenderingContext2d,
    pub cars: Rc<Vec<Rc<RefCell<Car>>>>,
    pub road: Road,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let car_canvas = document.get_element_by_id("carCanvas").unwrap();
        let car_canvas = car_canvas.dyn_into::<HtmlCanvasElement>()?;
        car_canvas.set_width(200);

        let car_context = car_canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let network_canvas = document.get_element_by_id("networkCanvas").unwrap();
        let network_canvas = network_canvas.dyn_into::<HtmlCanvasElement>()?;
        network_canvas.set_width(400);

        let network_context = network_canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let road = Road::create(
            car_context.clone(),
            car_canvas.width() as f64 / 2.0,
            car_canvas.width() as f64 * 0.9,
        );

        let car_count = 200;
        let cars = Rc::new(Self::generate_cars(&car_context, &road, car_count));

        Self {
            window,
            document,
            car_canvas,
            car_context,
            network_canvas,
            network_context,
            cars,
            road,
        }
        .ok()
    }

    fn generate_cars(
        car_context: &CanvasRenderingContext2d,
        road: &Road,
        number: usize,
    ) -> Vec<Rc<RefCell<Car>>> {
        (0..number)
            .flat_map(|_| {
                Car::create(
                    car_context.clone(),
                    Point2D::create(road.get_lane_center(1), 100.0),
                    30.0,
                    50.0,
                    ControlType::AI,
                )
            })
            .collect()
    }
}
