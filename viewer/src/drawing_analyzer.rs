use crate::html::HtmlDom;
use crate::html_draw::Draw;
use commons::geometry::Point2D;
use commons::math::normalize_points;
use drawing_commons::models::{DrawingPaths, Features};
use drawing_commons::sketch_pad::SketchPad;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_commons::chart::Chart;
use web_commons::html::Visibility;
use web_commons::subscribers::AddListener;
use web_sys::{window, HtmlElement, MouseEvent};

pub trait DrawingAnalyzer {
    fn toggle_input(&self) -> Result<(), JsValue>;
    fn toggle_output(&self) -> Result<(), JsValue>;
    fn subscribe_drawing_updates(
        &self,
        html: &Rc<RefCell<HtmlDom>>,
        min_max: &'static Vec<Vec<f64>>,
    );
}

impl DrawingAnalyzer for HtmlDom {
    fn toggle_input(&self) -> Result<(), JsValue> {
        let chart = self.chart.clone();
        let sketch_pad = self.sketch_pad.clone();

        handle_toggle_input(&chart, &sketch_pad)?;

        self.toggle_input_button
            .on_click(move |_event: MouseEvent| handle_toggle_input(&chart, &sketch_pad))
    }

    fn toggle_output(&self) -> Result<(), JsValue> {
        self.toggle_output_button
            .on_click(move |_event: MouseEvent| handle_toggle_output())
    }

    fn subscribe_drawing_updates(
        &self,
        html: &Rc<RefCell<HtmlDom>>,
        min_max: &'static Vec<Vec<f64>>,
    ) {
        let mut sketch_pad = self.sketch_pad.borrow_mut();

        let html = html.clone();
        let on_update_callback = Rc::new(RefCell::new(move |drawing: &DrawingPaths<Point2D>| {
            let point = drawing.get_feature(|x| x.x, |x| x.y);

            let point = normalize_points(&min_max[0], &min_max[1], vec![vec![point.x, point.y]]);
            let point = Point2D {
                x: point[0][0],
                y: point[0][1],
            };

            html.borrow().show_classified_point(Some(point)).expect("");
        }));

        sketch_pad.set_on_update(on_update_callback)
    }
}

fn handle_toggle_input(
    chart: &Rc<RefCell<Chart>>,
    sketch_pad: &Rc<RefCell<SketchPad>>,
) -> Result<(), JsValue> {
    let document = window().expect("").document().expect("");
    let container = document
        .get_element_by_id("inputContainer")
        .unwrap()
        .dyn_into::<HtmlElement>()?;

    let is_displayed = container.is_displayed();
    container.set_display(!is_displayed)?;
    if is_displayed {
        chart.borrow_mut().show_dynamic_point(None)?;
    } else {
        sketch_pad.borrow().trigger_update();
    }

    Ok(())
}

fn handle_toggle_output() -> Result<(), JsValue> {
    let document = window().expect("").document().expect("");
    let container = document
        .get_element_by_id("confusionContainer")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .expect("");

    let is_displayed = container.is_displayed();
    container.set_display(!is_displayed)
}
