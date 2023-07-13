mod html;

use crate::html::HtmlDom;
use drawing_commons::Sample;
use itertools::Itertools;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

lazy_static! {
    // TODO const variables don't work as arguments of std::include_str!
    static ref SAMPLES_DATA: Vec<Sample> =
        serde_json::from_str::<Vec<Sample>>(std::include_str!("../../data/dataset/samples.json"))
            .expect("");
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create()?;

    for (_id, group) in &SAMPLES_DATA.iter().group_by(|x| x.student_id) {
        let group = group.collect::<Vec<_>>();
        html.create_row(group[0].student_name.as_str(), group.as_slice())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::SAMPLES_DATA;

    #[test]
    fn test_samples() {
        let size = SAMPLES_DATA.len();
        assert_eq!(size, 5728);
    }
}
