use crate::sketch_pad::SketchPad;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart::Chart;
use web_commons::chart_models::{Options, SampleStyle, SampleStyleType};
use web_sys::{window, Document, Element, HtmlButtonElement};
use drawing_commons::classifiers::knn::KNN;
use drawing_commons::data::TRAINING_FEATURES;
use drawing_commons::ui::COLOR_PER_LABEL;

fn default_chart_options(feature_names: &[String]) -> Options {
    let mut styles = HashMap::<String, SampleStyle>::new();

    let mut insert_label = |label: &str, text: &str| {
        let color = COLOR_PER_LABEL.get(label).expect("");
        styles.insert(
            label.to_owned(),
            SampleStyle {
                color: (*color).to_owned(),
                text: text.to_owned(),
                image: None,
            },
        )
    };

    insert_label("car", "🚗");
    insert_label("fish", "🐟");
    insert_label("house", "🏠");
    insert_label("tree", "🌳");
    insert_label("bicycle", "🚲");
    insert_label("guitar", "🎸");
    insert_label("pencil", "✏️");
    insert_label("clock", "⏰");
    insert_label("?", "❓");

    Options {
        size: 500,
        axis_labels: [feature_names[0].clone(), feature_names[1].clone()],
        styles,
        icon: SampleStyleType::Image,
        transparency: Some(0.7),
    }
}

pub struct HtmlDom {
    pub document: Document,
    pub container: Element,
    pub statistics: Element,
    pub predicted_label_container: Element,
    pub control_panel_button: HtmlButtonElement,
    pub chart: Rc<RefCell<Chart>>,
    pub sketch_pad: Rc<RefCell<SketchPad>>,
    pub classifier: Rc<RefCell<KNN>>,
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

        let control_panel = document.get_element_by_id("controlPanel").unwrap();
        let control_panel_button = control_panel
            .query_selector("button")?
            .expect("")
            .dyn_into::<HtmlButtonElement>()?;

        let statistics = document.get_element_by_id("statistics").unwrap();

        let predicted_label_container = document
            .get_element_by_id("predictedLabelContainer")
            .unwrap();

        let sketch_pad = SketchPad::create("inputContainer")?;

        let testing_data = &TRAINING_FEATURES.features;
        let classifier = Rc::new(RefCell::new(KNN::new(testing_data, 50)));

        Self {
            document,
            container,
            statistics,
            predicted_label_container,
            control_panel_button,
            chart,
            sketch_pad,
            classifier,
        }
        .ok()
    }
}
