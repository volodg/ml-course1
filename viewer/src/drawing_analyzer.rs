use crate::html::HtmlDom;
use commons::math::Point;
use drawing_commons::models::{DrawingPaths, Features, FeaturesData};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_commons::html::AddListener;
use web_commons::html::Visibility;
use web_sys::{window, HtmlElement, MouseEvent};

pub trait DrawingAnalyzer {
    fn toggle_input(&self) -> Result<(), JsValue>;
    fn subscribe_drawing_updates(&self, feature_data: &'static FeaturesData);
}

impl DrawingAnalyzer for HtmlDom {
    fn toggle_input(&self) -> Result<(), JsValue> {
        let chart = self.chart.clone();
        let sketch_pad = self.sketch_pad.clone();

        self.control_panel_button
            .on_click(move |_event: MouseEvent| {
                let document = window().expect("").document().expect("");
                let container = document
                    .get_element_by_id("inputContainer")
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .expect("");

                let is_displayed = container.is_displayed();
                container.set_display(!is_displayed).expect("");
                if is_displayed {
                    chart.borrow_mut().show_dynamic_point(None).expect("");
                } else {
                    sketch_pad.borrow().trigger_update();
                }
            })
    }

    fn subscribe_drawing_updates(&self, feature_data: &'static FeaturesData) {
        let mut sketch_pad = self.sketch_pad.borrow_mut();
        let chart = self.chart.clone();

        let on_update_callback = Rc::new(RefCell::new(move |drawing: &DrawingPaths<Point>| {
            let point = drawing.get_feature(|x| x.x, |x| x.y);

            chart
                .borrow_mut()
                .show_dynamic_point(Some(point))
                .expect("");
        }));

        sketch_pad.set_on_update(on_update_callback)
    }
}

trait HtmlDomExt {
    fn classify(point: &Point, feature_data: &'static FeaturesData) -> String;
}

impl HtmlDomExt for HtmlDom {
    fn classify(_point: &Point, feature_data: &'static FeaturesData) -> String {
        let sample_points = feature_data
            .features
            .iter()
            .map(|x| Point {
                x: x.point[0] as f64,
                y: x.point[1] as f64,
            })
            .collect::<Vec<_>>();

        "".to_owned()
    }
}
