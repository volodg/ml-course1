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
use crate::storage::load_best_brain;

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let window = self.window.clone();
        let car_canvas = self.car_canvas.clone();
        let car_context = self.car_context.clone();
        let network_canvas = self.network_canvas.clone();
        let network_context = self.network_context.clone();
        let cars = self.cars.clone();
        let road = self.road.clone();

        if let Some(best_brain) = load_best_brain()? {
            cars[0].borrow_mut().brain = Some(best_brain);
        }

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

        let app_state = app_state.clone();
        animate_with_callback(move |time| {
            for car in &traffic {
                car.borrow_mut().update(&road.borders, &[]);
            }

            for car in cars.deref() {
                car.borrow_mut().update(&road.borders, &traffic);
            }

            car_canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);
            network_canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);

            car_context.save();
            let best_car_position = {
                let best_car_position = cars
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
                    -cars[best_car_position].borrow().position.y + car_canvas.height() as f64 * 0.7,
                )?;
                best_car_position
            };

            // save best car brain
            app_state.borrow_mut().best_car = cars[best_car_position].borrow().brain.clone();

            road.draw()?;

            for car in &traffic {
                car.borrow().draw("red", false)?;
            }

            car_context.set_global_alpha(0.2);
            for car in cars.deref() {
                car.borrow().draw("blue", false)?;
            }
            car_context.set_global_alpha(1.0);
            cars[best_car_position].borrow().draw("blue", true)?;

            car_context.restore();

            if let Some(brain) = &cars[best_car_position].borrow().brain {
                network_context.set_line_dash_offset(-time / 80.0);
                Visualizer::draw_network(&network_canvas, &network_context, brain)
            }

            Ok(())
        });

        Ok(())
    }
}
