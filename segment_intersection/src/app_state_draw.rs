use crate::app_state::AppState;
use crate::draw::DrawWithState;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

impl DrawWithState for AppState {
    fn draw(&self, app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.html.draw(app_state)
    }
}
