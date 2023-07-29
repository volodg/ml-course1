mod drawing_analyzer;
mod html;
mod html_draw;
mod sketch_pad;

use crate::drawing_analyzer::DrawingAnalyzer;
use crate::html::HtmlDom;
use crate::html_draw::Draw;
use commons::math::Point;
use drawing_commons::data::{FEATURES_DATA, MIN_MAX_DATA, TESTING_FEATURES, TRAINING_FEATURES};
use drawing_commons::models::SampleWithFeatures;
use itertools::Itertools;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::window;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create(&FEATURES_DATA.feature_names)?;

    fn add_rows(html: &HtmlDom, features: &[SampleWithFeatures]) -> Result<(), JsValue> {
        for (_, group) in &features.iter().group_by(|x| x.sample.student_id) {
            let group = group.collect::<Vec<_>>();
            html.create_row(group[0].sample.student_name.as_str(), group.as_slice())?;
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

    add_rows(&html, &TRAINING_FEATURES.features)?;

    let subtitle = window()
        .expect("")
        .document()
        .expect("")
        .create_element("h2")?;
    subtitle.set_inner_html("TESTING");
    html.container.append_child(&subtitle)?;

    add_rows(&html, &TESTING_FEATURES.read().expect("").features)?;

    html.plot_statistic(&TRAINING_FEATURES)?;

    // TODO update - MIN_MAX_DATA?
    html.subscribe_drawing_updates(&MIN_MAX_DATA);
    html.toggle_input()?;

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
