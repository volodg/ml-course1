use crate::app_state::CarType;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart::Chart;
use web_commons::chart_models::{Options, SampleStyle, SampleStyleType};
use web_sys::{window, Document, HtmlTableElement};

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
        size: 600,
        axis_labels: ["Kilometers".to_owned(), "Price".to_owned()],
        styles,
        icon: SampleStyleType::Image,
        transparency: Some(0.8),
        background: None,
    }
}

pub struct HtmlDom {
    pub document: Document,
    pub data_table: HtmlTableElement,
    pub chart: Rc<RefCell<Chart>>,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        let chart_container = document.get_element_by_id("chartContainer").unwrap();

        let data_table = document.get_element_by_id("dataTable").unwrap();
        let data_table = data_table.dyn_into::<HtmlTableElement>()?;

        let chart = Chart::create(chart_container.clone(), default_chart_options())?;

        Self {
            document,
            data_table,
            chart,
        }
        .ok()
    }
}
