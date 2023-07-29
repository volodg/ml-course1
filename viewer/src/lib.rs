mod drawing_analyzer;
mod html;
mod html_draw;
mod sketch_pad;

use crate::drawing_analyzer::DrawingAnalyzer;
use crate::html::HtmlDom;
use crate::html_draw::Draw;
use commons::math::Point;
use drawing_commons::knn_classifier::classify;
use drawing_commons::models::Sample;
use drawing_commons::models::{FeaturesData, SampleWithFeatures};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::window;

lazy_static! {
    // TODO const variables don't work as arguments of std::include_str!
    static ref SAMPLES_DATA: Vec<Sample> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/samples.json"))
            .expect("");
    static ref FEATURES_DATA: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/features.json"))
            .expect("");
    static ref TRAINING_DATA: Vec<Sample> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/training.json"))
            .expect("");
    static ref TESTING_DATA: Vec<Sample> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/testing.json"))
            .expect("");
    static ref TRAINING_FEATURES: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/training_features.json"))
            .expect("");
    static ref TESTING_FEATURES: RwLock<FeaturesData> =
        RwLock::new(serde_json::from_str::<_>(std::include_str!("../../data/dataset/testing_features.json"))
            .expect(""));
    static ref MIN_MAX_DATA: Vec<Vec<f64>> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/minMax.json"))
            .expect("");
}

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
            let (label, _) = classify(
                &Point {
                    x: feature.point[0],
                    y: feature.point[1],
                },
                &TRAINING_FEATURES.features,
                50,
            );
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
    html.subscribe_drawing_updates(&MIN_MAX_DATA, &TRAINING_FEATURES);
    html.toggle_input()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        FEATURES_DATA, MIN_MAX_DATA, SAMPLES_DATA, TESTING_DATA, TESTING_FEATURES, TRAINING_DATA,
        TRAINING_FEATURES,
    };

    #[test]
    fn test_resources() {
        let samples_count = 5728;

        let size = SAMPLES_DATA.len();
        assert_eq!(size, samples_count);

        let size = FEATURES_DATA.features.len();
        assert_eq!(size, samples_count);

        let size = TESTING_DATA.len();
        assert_eq!(size, 2864);

        let size = MIN_MAX_DATA.len();
        assert_eq!(size, 2);

        let size = TRAINING_DATA.len();
        assert_eq!(size, 2864);

        let size = TRAINING_FEATURES.features.len();
        assert_eq!(size, 2864);

        let size = TESTING_DATA.len();
        assert_eq!(size, 2864);

        let size = TESTING_FEATURES.read().expect("").features.len();
        assert_eq!(size, 2864);
    }
}
