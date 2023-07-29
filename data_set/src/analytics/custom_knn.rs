use crate::file_utils::print_progress;
use commons::math::Point;
use drawing_commons::classifiers::knn::KNN;
use drawing_commons::data::{TESTING_FEATURES, TRAINING_FEATURES};
use drawing_commons::ui::COLOR_PER_LABEL;
use image::{ImageBuffer, Rgb};
use std::io::ErrorKind;

#[allow(dead_code)]
pub fn run_knn_evaluations() -> Result<(), std::io::Error> {
    println!("RUNNING CLASSIFICATIONS");

    let training_samples = &TRAINING_FEATURES.features;

    let knn = KNN::new(training_samples, 50);

    let training_samples = &TESTING_FEATURES.read().expect("").features;

    let mut correct_count = 0;
    let mut total_count = 0;

    for sample in training_samples {
        let label = knn
            .predict(&Point {
                x: sample.point[0],
                y: sample.point[1],
            })
            .0;
        if label == sample.sample.label {
            correct_count += 1;
        }
        total_count += 1;
    }

    println!(
        "ACCURACY: {correct_count}/{total_count} ({:.2}%)",
        correct_count as f64 / total_count as f64 * 100.0
    );

    println!("GENERATING DECISION BOUNDARY");

    let mut image = ImageBuffer::new(5000, 5000);
    let total = (image.width() * image.height()) as usize;
    let mut current = 0 as usize;

    for x in 0..image.width() {
        for y in 0..image.height() {
            let point = Point {
                x: x as f64 / image.width() as f64,
                y: 1.0 - y as f64 / image.height() as f64,
            };
            let label = knn.predict(&point).0;
            let (r, g, b) = COLOR_PER_LABEL.get(label.as_str()).expect("").1;

            image.put_pixel(x, y, Rgb([r, g, b]));

            current += 1;
            print_progress("Generating image", current.into(), total.into())
        }
    }

    image
        .save("./data/dataset/decision_boundary.png")
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

    Ok(())
}
