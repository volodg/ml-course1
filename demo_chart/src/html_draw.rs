use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlTableRowElement, HtmlTableSectionElement};

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
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

        let body = self
            .data_table
            .create_t_body()
            .dyn_into::<HtmlTableSectionElement>()?;

        for sample in &app_state.samples {
            let tr = body.insert_row()?.dyn_into::<HtmlTableRowElement>()?;
            tr.insert_cell()?.set_inner_html(sample.id.to_string().as_str());
            tr.insert_cell()?.set_inner_html(sample.label.as_str());
            tr.insert_cell()?.set_inner_html(std::format!("sin = {:.0}", sample.km).as_str());
            tr.insert_cell()?.set_inner_html(std::format!("sin = {:.0}", sample.price).as_str());
        }

        Ok(())
    }
}
