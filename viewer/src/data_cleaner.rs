use lazy_static::lazy_static;
use std::sync::RwLock;
use web_commons::chart_models::Sample;

lazy_static! {
    pub static ref FLAGGED_SAMPLES: RwLock<Vec<Sample>> = RwLock::new(vec![]);
}

pub fn toggle_flagged_sample(sample: &Sample) {
    let mut samples = FLAGGED_SAMPLES.write().expect("");

    if samples.iter().find(|x| x.id == sample.id).is_some() {
        samples.retain(|x| x.id != sample.id);
    } else {
        samples.push(sample.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::data_cleaner::{toggle_flagged_sample, FLAGGED_SAMPLES};
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

        toggle_flagged_sample(&sample);
        assert_eq!(FLAGGED_SAMPLES.read().expect("").len(), original_count + 1);

        toggle_flagged_sample(&sample);
        assert_eq!(FLAGGED_SAMPLES.read().expect("").len(), original_count);
    }
}
