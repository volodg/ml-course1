use crate::app_state::{AppState, CarType};
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart::Chart;
use web_commons::chart_models::Options;
use web_sys::{HtmlTableRowElement, HtmlTableSectionElement};

fn default_chart_options() -> Options {
    let mut styles = HashMap::<String, String>::new();
    styles.insert(CarType::Basic.to_string(), "gray".to_owned());
    styles.insert(CarType::Sport.to_string(), "red".to_owned());
    Options {
        size: 500,
        axis_labels: ["Kilometers".to_owned(), "Price".to_owned()],
        styles,
    }
}

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
            tr.insert_cell()?
                .set_inner_html(sample.id.to_string().as_str());
            tr.insert_cell()?.set_inner_html(sample.label.as_str());
            tr.insert_cell()?
                .set_inner_html(std::format!("{:.0}", sample.point.x).as_str());
            tr.insert_cell()?
                .set_inner_html(std::format!("{:.0}", sample.point.y).as_str());
        }

        let chart = Chart::create(
            self.chart_container.clone(),
            app_state.samples.clone(),
            default_chart_options(),
        )?;
        chart.draw()
    }
}
