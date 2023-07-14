use crate::app_state::AppState;
use wasm_bindgen::JsValue;

pub trait Draw {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue>;

    fn redraw(&self, app_state: &AppState) -> Result<(), JsValue>;
}
