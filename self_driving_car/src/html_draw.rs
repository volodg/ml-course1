use crate::app_state::AppState;
use crate::car::{Car, ControlType};
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use commons::geometry::Point2D;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::animations::animate_with_callback;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let window = self.window.clone();
        let car_canvas = self.car_canvas.clone();
        let car_context = self.car_context.clone();
        let car = self.car.clone();
        let road = self.road.clone();

        let traffic = vec![Car::create_with_max_speed(
            car_context.clone(),
            Point2D {
                x: road.get_lane_center(1),
                y: -100.0,
            },
            30.0,
            50.0,
            ControlType::Dummy,
            2.0,
        )?];

        animate_with_callback(move || {
            for car in &traffic {
                car.borrow_mut().update(&road.borders, &[]);
            }

            let mut car = car.borrow_mut();
            car.update(&road.borders, &traffic);
            car_canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);

            car_context.save();
            car_context.translate(0.0, -car.position.y + car_canvas.height() as f64 * 0.7)?;

            road.draw()?;

            for car in &traffic {
                car.borrow().draw("red")?;
            }
            car.draw("blue")?;

            car_context.restore();
            Ok(())
        });

        Ok(())
    }
}
