mod drawing_analyzer;
mod html;
mod html_draw;
mod sketch_pad;

use crate::drawing_analyzer::DrawingAnalyzer;
use crate::html::HtmlDom;
use crate::html_draw::Draw;
use drawing_commons::models::FeaturesData;
use drawing_commons::models::Sample;
use itertools::Itertools;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

lazy_static! {
    // TODO const variables don't work as arguments of std::include_str!
    static ref SAMPLES_DATA: Vec<Sample> =
        serde_json::from_str::<Vec<_>>(std::include_str!("../../data/dataset/samples.json"))
            .expect("");
    static ref FEATURES_DATA: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/features.json"))
            .expect("");
    static ref TRAINING_DATA: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/training.json"))
            .expect("");
    static ref TESTING_DATA: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/testing.json"))
            .expect("");
    static ref MIN_MAX_DATA: Vec<Vec<f64>> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/minMax.json"))
            .expect("");
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create(&FEATURES_DATA.feature_names)?;

    html.plot_statistic(&FEATURES_DATA)?;

    for (_, group) in &SAMPLES_DATA.iter().group_by(|x| x.student_id) {
        let group = group.collect::<Vec<_>>();
        html.create_row(group[0].student_name.as_str(), group.as_slice())?;
    }

    html.subscribe_drawing_updates(&MIN_MAX_DATA, &FEATURES_DATA);
    html.toggle_input()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{FEATURES_DATA, MIN_MAX_DATA, SAMPLES_DATA, TESTING_DATA, TRAINING_DATA};

    #[test]
    fn test_resources() {
        let samples_count = 5728;

        let size = SAMPLES_DATA.len();
        assert_eq!(size, samples_count);

        let size = FEATURES_DATA.features.len();
        assert_eq!(size, samples_count);

        let size = FEATURES_DATA.feature_names.len();
        assert_eq!(size, 2);

        let size = MIN_MAX_DATA.len();
        assert_eq!(size, 2);

        let size = TRAINING_DATA.features.len();
        assert_eq!(size, 2864);

        let size = TESTING_DATA.features.len();
        assert_eq!(size, 2864);
    }
}
