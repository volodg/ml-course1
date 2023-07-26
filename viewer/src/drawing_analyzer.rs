use crate::html::HtmlDom;
use commons::math::Point;
use drawing_commons::models::{DrawingPaths, Features};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_commons::html::AddListener;
use web_commons::html::Visibility;
use web_sys::{window, HtmlElement, MouseEvent};

pub trait DrawingAnalyzer {
    fn toggle_input(&self) -> Result<(), JsValue>;
    fn subscribe_drawing_updates(&self);
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

    fn subscribe_drawing_updates(&self) {
        let mut sketch_pad = self.sketch_pad.borrow_mut();
        let chart = self.chart.clone();

        let on_update_callback = Rc::new(RefCell::new(move |drawing: &DrawingPaths<Point>| {
            let point = Point {
                x: drawing.path_count() as f64,
                y: drawing.point_count() as f64,
            };

            chart
                .borrow_mut()
                .show_dynamic_point(Some(point))
                .expect("");
        }));

        sketch_pad.set_on_update(on_update_callback)
    }
}
