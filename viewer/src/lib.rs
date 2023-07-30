mod confusion;
mod data_cleaner;
mod drawing_analyzer;
mod html;
mod html_draw;
mod images;

use crate::drawing_analyzer::DrawingAnalyzer;
use crate::html::HtmlDom;
use crate::html_draw::Draw;
use commons::math::Point;
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
        features: &[SampleWithFeatures],
        testing: bool,
    ) -> Result<(), JsValue> {
        for (_, group) in &features.iter().group_by(|x| x.sample.student_id) {
            let group = group.collect::<Vec<_>>();
            html.borrow().create_row(
                html,
                group[0].sample.student_name.as_str(),
                group.as_slice(),
                testing,
            )?;
        }
        Ok(())
    }

    {
        let mut correct_count = 0;
        let mut total_count = 0;

        let testing_data = &mut TESTING_FEATURES.write().expect("").features;
        for feature in testing_data.iter_mut() {
            let truth = feature.sample.label.clone();
            let (label, _) = html.classifier.borrow().predict(&Point {
                x: feature.point[0],
                y: feature.point[1],
            });
            let correct = truth == label;
            if correct {
                correct_count += 1;
            }
            total_count += 1;

            feature.truth = Some(truth);
            feature.sample.label = label;
            feature.correct = Some(correct)
        }

        html.statistics.set_inner_html(
            std::format!(
                "<b>ACCURACY</b><br>{correct_count}/{total_count} ({:.2}%)",
                correct_count as f64 / total_count as f64 * 100.0
            )
            .as_str(),
        );
    }

    let html = Rc::new(RefCell::new(html));
    add_rows(&html, &TRAINING_FEATURES.features, false)?;

    let subtitle = window()
        .expect("")
        .document()
        .expect("")
        .create_element("h2")?;
    subtitle.set_inner_html("TESTING");

    let html_ref = html.borrow();

    html_ref.container.append_child(&subtitle)?;

    add_rows(&html, &TESTING_FEATURES.read().expect("").features, true)?;

    {
        let chart_samples = &TESTING_FEATURES
            .read()
            .expect("")
            .features
            .iter()
            .map(|feature| Sample {
                id: feature.sample.id,
                label: feature.sample.label.clone(),
                point: Point {
                    x: feature.point[0],
                    y: feature.point[1],
                },
            })
            .collect::<Vec<_>>();

        html_ref.plot_statistic(&html, &chart_samples)?;
        html_ref
            .confusion
            .borrow_mut()
            .set_samples(&chart_samples, &CLASSES);
    }

    html_ref.subscribe_drawing_updates(&html, &MIN_MAX_DATA);
    html_ref.toggle_input()?;
    html_ref.toggle_output()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let size = 2 + 2;
        assert_eq!(size, 4);
    }
}
