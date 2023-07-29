extern crate core;

mod draw;
mod file_utils;

use crate::file_utils::{
    get_drawings_by_id, read_drawing_data, store_drawings_as_json, store_drawings_as_png,
    store_samples,
};
use commons::math::Point;
use drawing_commons::data::{TESTING_FEATURES, TRAINING_FEATURES};
use drawing_commons::knn_classifier::KNN;

#[allow(dead_code)]
fn build_data_set() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);
    store_drawings_as_json(&drawings)?;
    store_drawings_as_png(&drawings);

    Ok(())
}

#[allow(dead_code)]
fn run_evaluations() -> Result<(), std::io::Error> {
    let training_samples = &TRAINING_FEATURES.features;

    let knn = KNN::new(training_samples, 50);

    let training_samples = &TESTING_FEATURES.read().expect("").features;

    let mut correct_count = 0;
    let mut total_count = 0;

    for sample in training_samples {
        let label = knn.predict(&Point {
            x: sample.point[0],
            y: sample.point[1],
        });
        if label == sample.sample.label {
            correct_count += 1;
        }
        total_count += 1;
    }

    println!(
        "ACCURACY: {correct_count}/{total_count} ({:.2}%)",
        correct_count as f64 / total_count as f64 * 100.0
    );

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    println!("RUNNING CLASSIFICATIONS");

    run_evaluations()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
