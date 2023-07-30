use lazy_static::lazy_static;

use crate::models::FeaturesData;
use crate::models::Sample;

lazy_static! {
    // TODO const variables don't work as arguments of std::include_str!
    pub static ref SAMPLES_DATA: Vec<Sample> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/samples.json"))
            .expect("");
    pub static ref FEATURES_DATA: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/features.json"))
            .expect("");
    pub static ref TRAINING_DATA: Vec<Sample> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/training.json"))
            .expect("");
    pub static ref TESTING_DATA: Vec<Sample> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/testing.json"))
            .expect("");
    pub static ref TRAINING_FEATURES: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/training_features.json"))
            .expect("");
    pub static ref TESTING_FEATURES: FeaturesData =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/testing_features.json"))
            .expect("");
    pub static ref MIN_MAX_DATA: Vec<Vec<f64>> =
        serde_json::from_str::<_>(std::include_str!("../../data/dataset/minMax.json"))
            .expect("");
    pub static ref IMAGE_SRC: &'static str = "../../data/dataset/decision_boundary.png";
}

#[cfg(test)]
mod tests {
    use crate::data::{
        FEATURES_DATA, MIN_MAX_DATA, SAMPLES_DATA, TESTING_DATA, TESTING_FEATURES, TRAINING_DATA,
        TRAINING_FEATURES,
    };

    #[test]
    fn test_resources() {
        let samples_count = 4835;

        let size = SAMPLES_DATA.len();
        assert_eq!(size, samples_count);

        let size = FEATURES_DATA.features.len();
        assert_eq!(size, samples_count);

        let size = TESTING_DATA.len();
        assert_eq!(size, 2418);

        let size = MIN_MAX_DATA.len();
        assert_eq!(size, 2);

        let size = TRAINING_DATA.len();
        assert_eq!(size, 2417);

        let size = TRAINING_FEATURES.features.len();
        assert_eq!(size, 2417);

        let size = TESTING_DATA.len();
        assert_eq!(size, 2418);

        let size = TESTING_FEATURES.read().expect("").features.len();
        assert_eq!(size, 2418);
    }
}
