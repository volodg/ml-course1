use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsValue;
use web_commons::animations::request_animation_frame;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.canvas.set_width(200);
        self.canvas
            .set_height(self.window.inner_height()?.as_f64().expect("") as u32);

        let animation_f = Rc::new(RefCell::new(None));
        let animation_f_copy = animation_f.clone();

        let car = self.car.clone();

        *animation_f_copy.borrow_mut() = Some(Closure::new(move || {
            let mut car = car.borrow_mut();
            car.update();
            car.draw();

            request_animation_frame(animation_f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(animation_f_copy.borrow().as_ref().unwrap());

        Ok(())
    }
}
