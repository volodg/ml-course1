use crate::app_state::AppState;
use crate::car::{Car, ControlType};
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use crate::visualizer::Visualizer;
use commons::geometry::Point2D;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::animations::animate_with_callback;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let window = self.window.clone();
        let car_canvas = self.car_canvas.clone();
        let car_context = self.car_context.clone();
        let network_canvas = self.network_canvas.clone();
        let network_context = self.network_context.clone();
        let cars = self.cars.clone();
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

        // ???

        animate_with_callback(move |time| {
            for car in &traffic {
                car.borrow_mut().update(&road.borders, &[]);
            }

            for car in cars.deref() {
                // let mut car = car.borrow_mut();
                car.borrow_mut().update(&road.borders, &traffic);
            }

            car_canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);
            network_canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);

            car_context.save();
            let position = {
                let position = cars
                    .deref()
                    .iter()
                    .zip(0..)
                    .min_by(|(a, _), (b, _)| {
                        a.borrow().position.y.total_cmp(&b.borrow().position.y)
                    })
                    .map(|(_, i)| i)
                    .unwrap_or(0);
                car_context.translate(
                    0.0,
                    -cars[position].borrow().position.y + car_canvas.height() as f64 * 0.7,
                )?;
                position
            };

            road.draw()?;

            for car in &traffic {
                car.borrow().draw("red")?;
            }

            car_context.set_global_alpha(0.2);
            for car in cars.deref() {
                car.borrow().draw("blue")?;
            }
            car_context.set_global_alpha(1.0);
            cars[position].borrow().draw("blue")?;

            car_context.restore();

            if let Some(brain) = &cars[position].borrow().brain {
                network_context.set_line_dash_offset(-time / 80.0);
                Visualizer::draw_network(&network_canvas, &network_context, brain)
            }

            Ok(())
        });

        Ok(())
    }
}
