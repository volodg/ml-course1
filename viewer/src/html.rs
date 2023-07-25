use commons::utils::OkExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::chart::Chart;
use web_commons::chart_models::{Options, SampleStyle, SampleStyleType};
use web_sys::{window, Document, Element};

fn default_chart_options(feature_names: &[String]) -> Options {
    let mut styles = HashMap::<String, SampleStyle>::new();

    styles.insert(
        "car".to_owned(),
        SampleStyle {
            color: "gray".to_owned(),
            text: "🚗".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "fish".to_owned(),
        SampleStyle {
            color: "red".to_owned(),
            text: "🐟".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "house".to_owned(),
        SampleStyle {
            color: "yellow".to_owned(),
            text: "🏠".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "tree".to_owned(),
        SampleStyle {
            color: "green".to_owned(),
            text: "🌳".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "bicycle".to_owned(),
        SampleStyle {
            color: "cyan".to_owned(),
            text: "🚲".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "guitar".to_owned(),
        SampleStyle {
            color: "blue".to_owned(),
            text: "🎸".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "pencil".to_owned(),
        SampleStyle {
            color: "magenta".to_owned(),
            text: "✏️".to_owned(),
            image: None,
        },
    );
    styles.insert(
        "clock".to_owned(),
        SampleStyle {
            color: "lightgray".to_owned(),
            text: "⏰".to_owned(),
            image: None,
        },
    );

    Options {
        size: 400,
        axis_labels: [feature_names[0].clone(), feature_names[1].clone()],
        styles,
        icon: SampleStyleType::Image,
        transparency: Some(0.7),
    }
}

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub container: Element,
    pub chart: Rc<RefCell<Chart>>,
}

impl HtmlDom {
    pub fn create(feature_names: &[String]) -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let container = document.get_element_by_id("container").unwrap();

        let chart_container = document.get_element_by_id("chartContainer").unwrap();
        let chart = Chart::create(
            chart_container.clone(),
            default_chart_options(feature_names),
        )?;

        Self {
            document,
            container,
            chart,
        }
        .ok()
    }
}
