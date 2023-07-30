use crate::data_cleaner::toggle_flagged_sample;
use crate::html::HtmlDom;
use commons::math::Point;
use drawing_commons::models::{FeaturesData, SampleWithFeatures};
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart_models::Sample;
use web_commons::document::DocumentExt;
use web_commons::html::AddListener;
use web_commons::log;
use web_sys::{
    window, HtmlElement, HtmlImageElement, MouseEvent, ScrollBehavior, ScrollIntoViewOptions,
    ScrollLogicalPosition,
};

pub trait Draw {
    fn create_row(
        &self,
        html: &Rc<RefCell<HtmlDom>>,
        student_name: &str,
        features: &[&SampleWithFeatures],
        testing: bool,
    ) -> Result<(), JsValue>;
    fn plot_statistic(
        &self,
        html: &Rc<RefCell<HtmlDom>>,
        feature_data: &FeaturesData,
    ) -> Result<(), JsValue>;
    fn show_classified_point(&self, point: Option<Point>) -> Result<(), JsValue>;
}

impl Draw for HtmlDom {
    fn create_row(
        &self,
        html: &Rc<RefCell<HtmlDom>>,
        student_name: &str,
        features: &[&SampleWithFeatures],
        testing: bool,
    ) -> Result<(), JsValue> {
        let row = self.document.create_element("div")?;
        row.class_list().add_1("row")?;
        _ = self.container.append_child(&row)?;

        let row_label = self.document.create_element("div")?;
        row_label.set_inner_html(student_name);
        row_label.class_list().add_1("rowLabel")?;
        _ = row.append_child(&row_label)?;

        for feature in features {
            let img = self
                .document
                .create_element("img")?
                .dyn_into::<HtmlImageElement>()?;

            let sample_container = self
                .document
                .create_element("div")?
                .dyn_into::<HtmlElement>()?;
            sample_container.set_id(std::format!("sample_{}", feature.sample.id).as_str());

            let sample = Sample {
                id: feature.sample.id,
                label: feature.sample.label.clone(),
                point: Point {
                    x: feature.point[0],
                    y: feature.point[1],
                },
            };
            let html = html.clone();
            sample_container.on_click(move |event: MouseEvent| {
                log(std::format!("{:?}", event.to_string()).as_str());
                if event.ctrl_key() {
                    toggle_flagged_sample(&sample);
                } else {
                    handle_click(&html, Some(&sample), false, testing).expect("");
                }
            })?;

            _ = sample_container.class_list().add_1("sampleContainer")?;
            if feature.correct.unwrap_or(false) {
                sample_container
                    .style()
                    .set_property("background-color", "#006")?;
            }

            let sample_label = self.document.create_element("div")?;
            sample_label.set_inner_html(feature.sample.label.as_str());
            _ = sample_container.append_child(&sample_label)?;

            let path = std::format!("{}/{}.png", IMG_DIR, feature.sample.id);
            img.set_src(path.as_str());
            img.class_list().add_1("thumb")?;
            if FLAGGED_USERS.contains(&feature.sample.student_id) {
                img.class_list().add_1("blur")?;
            }
            sample_container.append_child(&img)?;

            row.append_child(&sample_container)?;
        }

        Ok(())
    }

    fn plot_statistic(
        &self,
        html: &Rc<RefCell<HtmlDom>>,
        feature_data: &FeaturesData,
    ) -> Result<(), JsValue> {
        let mut chart = self.chart.borrow_mut();

        let samples = feature_data
            .features
            .iter()
            .zip(1..)
            .map(|(feature, id)| Sample {
                id,
                label: feature.sample.label.clone(),
                point: Point {
                    x: feature.point[0],
                    y: feature.point[1],
                },
            })
            .collect::<Vec<_>>();

        chart.set_samples(samples);

        let html = html.clone();
        let on_click_callback = Rc::new(RefCell::new(move |sample: Option<&Sample>| {
            handle_click(&html, sample, true, false).expect("")
        }));

        chart.set_on_click(on_click_callback);

        chart.draw()
    }

    fn show_classified_point(&self, point: Option<Point>) -> Result<(), JsValue> {
        let selection = match point {
            Some(point) => {
                let predicted_label_container = self.predicted_label_container.clone();
                let classifier = self.classifier.clone();

                let (label, samples) = classifier.borrow().predict(&point);
                predicted_label_container
                    .set_inner_html(std::format!("Is it a {:?} ?", label).as_str());
                let samples = samples
                    .into_iter()
                    .map(|feature| Sample {
                        id: feature.sample.id,
                        label: feature.sample.label,
                        point: Point {
                            x: feature.point[0],
                            y: feature.point[1],
                        },
                    })
                    .collect();

                Some((point, label, samples))
            }
            None => None,
        };

        self.chart.borrow_mut().show_dynamic_point(selection)
    }
}

fn handle_click(
    html: &Rc<RefCell<HtmlDom>>,
    sample: Option<&Sample>,
    scroll: bool,
    testing: bool,
) -> Result<(), JsValue> {
    let document = window().expect("").document().expect("");
    let emphasize_class_name = "emphasize";

    let de_emphasize =
        || -> Result<(), JsValue> { document.remove_all_classes(emphasize_class_name) };

    let (sample, point): (_, Option<Point>) = match sample {
        Some(sample) => {
            let element = document
                .get_element_by_id(std::format!("sample_{}", sample.id).as_str())
                .unwrap();

            if element.class_list().contains(emphasize_class_name) {
                element.class_list().remove_1(emphasize_class_name)?;
                (None, None)
            } else {
                de_emphasize()?;

                element.class_list().add_1(emphasize_class_name)?;

                if scroll {
                    let mut options = ScrollIntoViewOptions::new();
                    options.behavior(ScrollBehavior::Auto);
                    options.block(ScrollLogicalPosition::Center);
                    element.scroll_into_view_with_scroll_into_view_options(&options);
                }

                if testing {
                    (None, Some(sample.point.clone()))
                } else {
                    (Some(sample), None)
                }
            }
        }
        None => {
            de_emphasize()?;
            (None, None)
        }
    };

    html.borrow().chart.borrow_mut().select_sample(sample)?;
    html.borrow().show_classified_point(point)?;

    Ok(())
}
