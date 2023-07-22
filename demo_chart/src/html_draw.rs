use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlTableRowElement, HtmlTableSectionElement};
use web_commons::log;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &AppState) -> Result<(), JsValue> {
        // self.canvas.draw(app_state)?;

        let header = self
            .data_table
            .create_t_head()
            .dyn_into::<HtmlTableSectionElement>()?;
        let tr = header.insert_row()?.dyn_into::<HtmlTableRowElement>()?;
        tr.insert_cell()?.set_inner_html("Id");
        tr.insert_cell()?.set_inner_html("Type");
        tr.insert_cell()?.set_inner_html("Km");
        tr.insert_cell()?.set_inner_html("Price");
        log("HERE1");

        Ok(())
    }
}
