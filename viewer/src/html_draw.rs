use crate::html::HtmlDom;
use commons::math::Point;
use drawing_commons::models::{FeaturesData, Sample, SampleWithFeatures};
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart::Chart;
use web_commons::html::AddListener;
use web_sys::{
    window, Element, HtmlImageElement, MouseEvent, ScrollBehavior, ScrollIntoViewOptions,
    ScrollLogicalPosition,
};

pub trait Draw {
    fn create_row(
        &self,
        student_name: &str,
        features: &[&SampleWithFeatures],
    ) -> Result<(), JsValue>;
    fn plot_statistic(&self, feature_data: &FeaturesData) -> Result<(), JsValue>;
}

impl Draw for HtmlDom {
    fn create_row(
        &self,
        student_name: &str,
        features: &[&SampleWithFeatures],
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

            let sample_container = self.document.create_element("div")?;
            sample_container.set_id(std::format!("sample_{}", feature.sample.id).as_str());

            let chart = self.chart.clone();
            let sample = web_commons::chart_models::Sample {
                id: feature.sample.id,
                label: feature.sample.label.clone(),
                point: Point {
                    x: feature.point[0],
                    y: feature.point[1],
                },
            };
            sample_container.on_click(move |_event: MouseEvent| {
                handle_click(&chart, Some(&sample), false).expect("");
            })?;

            _ = sample_container.class_list().add_1("sampleContainer")?;

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

    fn plot_statistic(&self, feature_data: &FeaturesData) -> Result<(), JsValue> {
        let mut chart = self.chart.borrow_mut();

        use web_commons::chart_models::Sample;

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

        let on_click_chart = self.chart.clone();
        let on_click_callback = Rc::new(RefCell::new(move |sample: Option<&Sample>| {
            handle_click(&on_click_chart, sample, true).expect("")
        }));

        chart.set_on_click(on_click_callback);

        chart.draw()
    }
}

fn handle_click(
    chart: &Rc<RefCell<Chart>>,
    sample: Option<&web_commons::chart_models::Sample>,
    scroll: bool,
) -> Result<(), JsValue> {
    let document = window().expect("").document().expect("");
    let selected = document.query_selector_all(".emphasize")?;

    let emphasize_class_name = "emphasize";

    let de_emphasize = || -> Result<(), JsValue> {
        for i in 0..selected.length() {
            let element = selected.item(i).expect("").dyn_into::<Element>()?;
            element.class_list().remove_1(emphasize_class_name)?;
        }
        Ok(())
    };

    let sample = match sample {
        Some(sample) => {
            let element = document
                .get_element_by_id(std::format!("sample_{}", sample.id).as_str())
                .unwrap();

            if element.class_list().contains(emphasize_class_name) {
                element.class_list().remove_1(emphasize_class_name)?;
                None
            } else {
                de_emphasize()?;

                element.class_list().add_1(emphasize_class_name)?;

                if scroll {
                    let mut options = ScrollIntoViewOptions::new();
                    options.behavior(ScrollBehavior::Auto);
                    options.block(ScrollLogicalPosition::Center);
                    element.scroll_into_view_with_scroll_into_view_options(&options);
                }

                Some(sample)
            }
        }
        None => {
            de_emphasize()?;
            None
        }
    };

    chart.borrow_mut().select_sample(sample)?;

    Ok(())
}
