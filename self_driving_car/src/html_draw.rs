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
        let canvas = self.canvas.clone();
        let context = self.context.clone();
        let car = self.car.clone();
        let road = self.road.clone();

        let traffic = Box::new(vec![Car::create_with_max_speed(
            context.clone(),
            Point2D {
                x: road.get_lane_center(1),
                y: -100.0,
            },
            30.0,
            50.0,
            ControlType::Dummy,
            2.0,
        )?]);

        animate_with_callback(move || {
            for car in traffic.as_ref() {
                car.borrow_mut().update(&road.borders);
            }

            let mut car = car.borrow_mut();
            car.update(&road.borders);
            canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);

            context.save();
            context.translate(0.0, -car.position.y + canvas.height() as f64 * 0.7)?;

            road.draw()?;

            for car in traffic.as_ref() {
                car.borrow().draw()?;
            }
            car.draw()?;

            context.restore();
            Ok(())
        });

        Ok(())
    }
}
