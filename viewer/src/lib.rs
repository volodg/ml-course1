mod html;
mod html_draw;

use crate::html::HtmlDom;
use drawing_commons::models::FeaturesData;
use drawing_commons::models::Sample;
use itertools::Itertools;
use js_sys::eval;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use web_sys::HtmlScriptElement;
use crate::html_draw::{Draw, plot_statistic};

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
    let html = HtmlDom::create()?;

    {
        // set_inner_html_with_script
        let chart2 = html.document.get_element_by_id("chartContainer").unwrap();
        let chart = plot_statistic(&FEATURES_DATA);
        chart2.set_inner_html(&chart);

        let collection = chart2.get_elements_by_tag_name("script");
        for i in 0..collection.length() {
            let script = collection.item(i).expect("").dyn_into::<HtmlScriptElement>()?;
            let text: &str = &script.text().unwrap();
            eval(text)?;
        }
    }

    for (_id, group) in &SAMPLES_DATA.iter().group_by(|x| x.student_id) {
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
