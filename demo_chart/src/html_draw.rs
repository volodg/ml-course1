use crate::app_state::{AppState, CarType};
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart::Chart;
use web_commons::chart_models::{Options, Sample, SampleStyle, SampleStyleType};
use web_sys::{HtmlTableRowElement, HtmlTableSectionElement, ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition};

fn default_chart_options() -> Options {
    let mut styles = HashMap::<String, SampleStyle>::new();
    styles.insert(
        CarType::Basic.to_string(),
        SampleStyle {
            color: "blue".to_owned(),
            text: "🚗".to_owned(),
            image: None,
        },
    );
    styles.insert(
        CarType::Sport.to_string(),
        SampleStyle {
            color: "gray".to_owned(),
            text: "🏎".to_owned(),
            image: None,
        },
    );
    Options {
        size: 500,
        axis_labels: ["Kilometers".to_owned(), "Price".to_owned()],
        styles,
        icon: SampleStyleType::Image,
    }
}

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
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
            tr.set_id(sample.element_id().as_str());
            tr.insert_cell()?
                .set_inner_html(sample.id.to_string().as_str());
            tr.insert_cell()?.set_inner_html(sample.label.as_str());
            tr.insert_cell()?
                .set_inner_html(std::format!("{:.0}", sample.point.x).as_str());
            tr.insert_cell()?
                .set_inner_html(std::format!("{:.0}", sample.point.y).as_str());
        }

        let document = self.document.clone();
        let on_click_callback = move |sample: &Sample| {
            let element = document.get_element_by_id(sample.element_id().as_str()).expect("");
            element.class_list().add_1("emphasize").expect("");

            let mut options = ScrollIntoViewOptions::new();
            options.behavior(ScrollBehavior::Auto);
            options.block(ScrollLogicalPosition::Center);
            element.scroll_into_view_with_scroll_into_view_options(&options);
        };

        let chart = Chart::create(
            self.chart_container.clone(),
            app_state.samples.clone(),
            default_chart_options(),
            Some(on_click_callback),
        )?;
        let borrow_chart = chart.borrow();
        borrow_chart.draw()
    }
}

trait SampleExt {
    fn element_id(&self) -> String;
}

impl SampleExt for Sample {
    fn element_id(&self) -> String {
        std::format!("sample_{}", self.id)
    }
}
