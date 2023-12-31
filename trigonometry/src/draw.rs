use crate::app_state::AppState;
use wasm_bindgen::JsValue;

pub trait Draw {
    fn draw(&self) -> Result<(), JsValue>;
}

pub trait DrawWithState {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue>;
}
