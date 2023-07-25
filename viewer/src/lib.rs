mod html;
mod html_draw;

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
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create(&FEATURES_DATA.feature_names)?;

    html.plot_statistic(&FEATURES_DATA)?;

    for (_, group) in &SAMPLES_DATA.iter().group_by(|x| x.student_id) {
        let group = group.collect::<Vec<_>>();
        html.create_row(group[0].student_name.as_str(), group.as_slice())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{FEATURES_DATA, SAMPLES_DATA};

    #[test]
    fn test_resources() {
        let samples_count = 5728;

        let size = SAMPLES_DATA.len();
        assert_eq!(size, samples_count);

        let size = FEATURES_DATA.features.len();
        assert_eq!(size, samples_count);

        let size = FEATURES_DATA.feature_names.len();
        assert_eq!(size, 2);
    }
}
