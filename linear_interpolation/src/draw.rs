use crate::app_state::AppState;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

pub trait Draw {
    fn draw(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

pub trait DrawWithState {
    fn draw(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}
