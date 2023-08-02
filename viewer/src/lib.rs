mod confusion;
mod data_cleaner;
mod drawing_analyzer;
mod html;
mod html_draw;
mod images;
mod models;

use crate::drawing_analyzer::DrawingAnalyzer;
use crate::html::HtmlDom;
use crate::html_draw::Draw;
use crate::models::feature_to_chart_sample;
use drawing_commons::classifiers::knn::KNN;
use drawing_commons::data::{FEATURES_DATA, MIN_MAX_DATA, TESTING_FEATURES, TRAINING_FEATURES};
use drawing_commons::models::SampleWithFeatures;
use drawing_commons::utils::CLASSES;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_commons::chart_models::Sample;
use web_sys::window;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create(&FEATURES_DATA.feature_names)?;

    fn add_rows(
        html: &Rc<RefCell<HtmlDom>>,
        samples: &[Sample],
        testing: bool,
    ) -> Result<(), JsValue> {
        for (_, group) in &samples.iter().group_by(|x| x.group_id) {
            let group = group.collect::<Vec<_>>();
            html.borrow().create_row(
                html,
                group[0].group_name.as_str(),
                group.as_slice(),
                testing,
            )?;
        }
        Ok(())
    }

    let testing_chart_samples =
        features_to_chart_samples(&TESTING_FEATURES.features, Some(&html.classifier.borrow()));

    {
        let correct_count =
            testing_chart_samples.iter().fold(
                0,
                |acc, el| {
                    if el.correct() {
                        acc + 1
                    } else {
                        acc
                    }
                },
            );
        let total_count = testing_chart_samples.len();

        html.statistics.set_inner_html(
            std::format!(
                "<b>ACCURACY</b><br>{correct_count}/{total_count} ({:.2}%)",
                correct_count as f64 / total_count as f64 * 100.0
            )
            .as_str(),
        );
    }

    let html = Rc::new(RefCell::new(html));
    let training_samples = features_to_chart_samples(&TRAINING_FEATURES.features, None);
    add_rows(&html, &training_samples, false)?;

    let subtitle = window()
        .expect("")
        .document()
        .expect("")
        .create_element("h2")?;
    subtitle.set_inner_html("TESTING");

    let html_ref = html.borrow();

    html_ref.container.append_child(&subtitle)?;

    add_rows(&html, &testing_chart_samples, true)?;

    {
        // html_ref.plot_statistic(&html, &testing_chart_samples)?;
        html_ref.plot_statistic(&html, &vec![])?;
        html_ref
            .confusion
            .borrow_mut()
            .set_samples(&testing_chart_samples, &CLASSES);

        html_ref.confusion.borrow().draw()?;
    }

    html_ref.subscribe_drawing_updates(&html, &MIN_MAX_DATA);
    html_ref.toggle_input()?;
    html_ref.toggle_output()?;

    Ok(())
}

fn features_to_chart_samples(
    features: &[SampleWithFeatures],
    classifier: Option<&KNN>,
) -> Vec<Sample> {
    features
        .iter()
        .map(|feature| {
            let mut sample = feature_to_chart_sample(feature.clone());

            let (truth, label) = match classifier {
                Some(classifier) => {
                    let truth = feature.sample.label.clone();
                    let (label, _) = classifier.predict(&feature.point);
                    (Some(truth), label)
                }
                None => (None, feature.sample.label.clone()),
            };

            sample.truth = truth;
            sample.label = label;

            sample
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let size = 2 + 2;
        assert_eq!(size, 4);
    }
}
