extern crate core;

mod draw;
mod file_utils;

use crate::file_utils::{
    get_drawings_by_id, read_drawing_data, store_drawings_as_json, store_drawings_as_png,
    store_samples,
};
use ::draw::render::bitmap::PNGRenderer;
use ::draw::render::Renderer;
use ::draw::{render, Canvas, Color, Drawing, Shape, Style, SvgRenderer, RGB};
use commons::math::Point;
use drawing_commons::classifiers::knn::KNN;
use drawing_commons::data::{TESTING_FEATURES, TRAINING_FEATURES};
use drawing_commons::ui::COLOR_PER_LABEL;

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
            let (r, g, b) = COLOR_PER_LABEL.get(label.as_str()).expect("").1;

            let rect = Drawing::new()
                .with_shape(Shape::Rectangle {
                    width: 1,
                    height: 1,
                })
                .with_xy(x as f32, y as f32)
                .with_style(Style::stroked(1, RGB::new(r, g, b)));

            canvas.display_list.add(rect);
        }
    }

    render::save(
        &canvas,
        "./data/dataset/decision_boundary.svg",
        SvgRenderer::new(),
    )?;

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
