use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::RwLock;
use wasm_bindgen::JsValue;
use web_commons::chart_models::Sample;
use web_commons::document::DocumentExt;
use web_commons::log;
use web_sys::window;

lazy_static! {
    pub static ref FLAGGED_SAMPLES: RwLock<HashSet<usize>> = RwLock::new(HashSet::new());
}

fn toggle_flagged_sample_model(sample: &Sample) {
    let mut samples = FLAGGED_SAMPLES.write().expect("");

    if samples.contains(&sample.id) {
        samples.remove(&sample.id);
    } else {
        samples.insert(sample.id);
    }
}

pub fn toggle_flagged_sample(sample: &Sample) -> Result<(), JsValue> {
    toggle_flagged_sample_model(sample);

    let document = window().expect("").document().expect("");
    let class_name = "flagged";
    document.remove_all_classes(class_name)?;

    {
        let samples = FLAGGED_SAMPLES.read().expect("");
        for sample_id in samples.iter() {
            let element = document
                .get_element_by_id(std::format!("sample_{}", sample_id).as_str())
                .unwrap();

            element.class_list().add_1(class_name)?;
        }
        log(std::format!("samples: {:?}", samples).as_str())
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::data_cleaner::{toggle_flagged_sample_model, FLAGGED_SAMPLES};
    use commons::math::Point;
    use web_commons::chart_models::Sample;

    #[test]
    fn it_toggle_flagged_sample() {
        let sample = Sample {
            id: usize::MAX,
            label: "".to_owned(),
            point: Point::default(),
        };

        let original_count = FLAGGED_SAMPLES.read().expect("").len();

        toggle_flagged_sample_model(&sample);
        assert_eq!(FLAGGED_SAMPLES.read().expect("").len(), original_count + 1);

        toggle_flagged_sample_model(&sample);
        assert_eq!(FLAGGED_SAMPLES.read().expect("").len(), original_count);
    }
}
