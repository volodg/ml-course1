use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::log;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        log("draw me");

        self.canvas.set_width(200);
        self.canvas
            .set_height(self.window.inner_height()?.as_f64().expect("") as u32);

        self.car.draw();

        Ok(())
    }
}
