use crate::app_state::AppState;
use crate::draw::{Draw, DrawWithState};
use wasm_bindgen::JsValue;

impl Draw for AppState {
    fn draw(&self) -> Result<(), JsValue> {
        self.html.draw(self)
    }

    fn redraw(&self) -> Result<(), JsValue> {
        self.html.redraw(self)
    }
}
