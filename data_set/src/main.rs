extern crate core;

mod draw;
mod file_utils;

use crate::file_utils::{
    get_drawings_by_id, read_drawing_data, store_drawings_as_json, store_drawings_as_png,
    store_samples,
};
use ::draw::{render, Canvas, Color, Drawing, Shape, Style, SvgRenderer};
use commons::math::Point;
use drawing_commons::classifiers::knn::KNN;
use drawing_commons::data::{TESTING_FEATURES, TRAINING_FEATURES};

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

    // create a canvas to draw on
    let mut canvas = Canvas::new(100, 100);

    for x in 0..canvas.width {
        for y in 0..canvas.height {
            let point = Point {
                x: x as f64 / canvas.width as f64,
                y: 1.0 - y as f64 / canvas.height as f64,
            };
            let label = knn.predict(&point).0;
        }
    }

    // create a new drawing
    let rect = Drawing::new()
        // give it a shape
        .with_shape(Shape::Rectangle {
            width: 50,
            height: 50,
        })
        // move it around
        .with_xy(25.0, 25.0)
        // give it a cool style
        .with_style(Style::stroked(5, Color::black()));

    // add it to the canvas
    canvas.display_list.add(rect);

    // save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/basic_end_to_end.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save");

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    run_evaluations()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
