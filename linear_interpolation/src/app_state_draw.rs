use crate::app_state::AppState;
use crate::draw::{Draw, DrawWithState};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

impl Draw for AppState {
    fn draw(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.html.draw(app_state)
    }
}
