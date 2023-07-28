use crate::html::HtmlDom;
use crate::sketch_pad::SketchPad;
use commons::math::{normalize_points, Point};
use drawing_commons::models::{DrawingPaths, Features, FeaturesData, SampleWithFeatures};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_commons::chart::Chart;
use web_commons::chart_models::Sample;
use web_commons::html::AddListener;
use web_commons::html::Visibility;
use web_sys::{window, HtmlElement, MouseEvent};

pub trait DrawingAnalyzer {
    fn toggle_input(&self) -> Result<(), JsValue>;
    fn subscribe_drawing_updates(
        &self,
        min_max: &'static Vec<Vec<f64>>,
        feature_data: &'static FeaturesData,
    );
}

impl DrawingAnalyzer for HtmlDom {
    fn toggle_input(&self) -> Result<(), JsValue> {
        let chart = self.chart.clone();
        let sketch_pad = self.sketch_pad.clone();

        handle_toggle_input(&chart, &sketch_pad)?;

        self.control_panel_button
            .on_click(move |_event: MouseEvent| {
                handle_toggle_input(&chart, &sketch_pad).expect("");
            })
    }

    fn subscribe_drawing_updates(
        &self,
        min_max: &'static Vec<Vec<f64>>,
        feature_data: &'static FeaturesData,
    ) {
        let mut sketch_pad = self.sketch_pad.borrow_mut();
        let chart = self.chart.clone();
        let predicted_label_container = self.predicted_label_container.clone();

        let on_update_callback = Rc::new(RefCell::new(move |drawing: &DrawingPaths<Point>| {
            let point = drawing.get_feature(|x| x.x, |x| x.y);

            let point = normalize_points(&min_max[0], &min_max[1], vec![vec![point.x, point.y]]);
            let point = Point {
                x: point[0][0],
                y: point[0][1],
            };

            let (label, samples) = classify(&point, feature_data);
            predicted_label_container
                .set_inner_html(std::format!("Is it a {:?} ?", label).as_str());
            let samples = samples
                .into_iter()
                .map(|sample| Sample {
                    id: 0,
                    label: sample.label,
                    point: Point {
                        x: sample.point[0] as f64,
                        y: sample.point[1] as f64,
                    },
                })
                .collect();

            chart
                .borrow_mut()
                .show_dynamic_point(Some((point, label, samples)))
                .expect("");
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
        .dyn_into::<HtmlElement>()
        .expect("");

    let is_displayed = container.is_displayed();
    container.set_display(!is_displayed).expect("");
    if is_displayed {
        chart.borrow_mut().show_dynamic_point(None)?;
    } else {
        sketch_pad.borrow().trigger_update();
    }

    Ok(())
}

fn classify(
    point: &Point,
    feature_data: &'static FeaturesData,
) -> (String, Vec<SampleWithFeatures>) {
    let sample_points = feature_data
        .features
        .iter()
        .map(|x| Point {
            x: x.point[0],
            y: x.point[1],
        })
        .collect::<Vec<_>>();

    let indices = point.get_nearest_k(&sample_points, 10);

    let nearest_samples = indices
        .iter()
        .map(|i| feature_data.features[*i].clone())
        .collect::<Vec<_>>();

    let (_, (_, label)) = nearest_samples.iter().map(|x| x.label.clone()).fold(
        (HashMap::new(), (0, "".to_owned())),
        |(mut map, (frequency, label)), val| {
            let new_frequency = *map
                .entry(val.clone())
                .and_modify(|frq| *frq += 1)
                .or_insert(1);

            if new_frequency > frequency {
                (map, (new_frequency, val))
            } else {
                (map, (frequency, label))
            }
        },
    );

    (label, nearest_samples)
}
