use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlTableSectionElement;

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.canvas.draw(app_state)?;

        let header = self
            .data_table
            .create_t_head()
            .dyn_into::<HtmlTableSectionElement>()?;
        let _tr = header.insert_row();

        Ok(())
    }
}
