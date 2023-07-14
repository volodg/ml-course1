use crate::app_state::AppState;
use crate::draw::Draw;
use wasm_bindgen::JsValue;

impl Draw for AppState {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.html.draw(app_state)
    }

    fn redraw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.html.redraw(app_state)
    }
}
