use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use wasm_bindgen::JsValue;

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.canvas.draw(app_state)
    }
}
