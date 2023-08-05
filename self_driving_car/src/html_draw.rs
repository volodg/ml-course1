use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::animations::animate_with_callback;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.canvas.set_width(200);

        let window = self.window.clone();
        let canvas = self.canvas.clone();
        let car = self.car.clone();
        let road = self.road.clone();

        animate_with_callback(move || {
            let mut car = car.borrow_mut();
            car.update();
            canvas.set_height(window.inner_height()?.as_f64().expect("") as u32);

            car.draw()?;
            road.draw();

            Ok(())
        });

        Ok(())
    }
}
