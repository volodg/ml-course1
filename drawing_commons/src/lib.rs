pub mod classifiers;
pub mod data;
pub mod models;
pub mod ui;

use const_format::concatcp;

const DATA_DIR: &str = "./data";
pub const RAW_DIR: &str = concatcp!(DATA_DIR, "/raw");
const DATASET_DIR: &str = concatcp!(DATA_DIR, "/dataset");
pub const JSON_DIR: &str = concatcp!(DATASET_DIR, "/json");
pub const IMG_DIR: &str = concatcp!(DATASET_DIR, "/img");
pub const SAMPLES: &str = concatcp!(DATASET_DIR, "/samples.json");
pub const FEATURES: &str = concatcp!(DATASET_DIR, "/features.json");
pub const MIN_MAX_JS: &str = concatcp!(DATASET_DIR, "/minMax.json");
pub const TRAINING: &str = concatcp!(DATASET_DIR, "/training.json");
pub const TESTING: &str = concatcp!(DATASET_DIR, "/testing.json");
pub const TRAINING_FEATURES: &str = concatcp!(DATASET_DIR, "/training_features.json");
pub const TESTING_FEATURES: &str = concatcp!(DATASET_DIR, "/testing_features.json");

pub const FLAGGED_USERS: &[u64; 3] = &[1663882102141, 1663900040545, 1664485938220];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
