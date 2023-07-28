use crate::draw::generate_image_file;
use commons::math::{normalize_points, normalize_points_to_min_max};
use drawing_commons::models::{
    get_feature_names, DrawingData, DrawingPaths, Features, FeaturesData, Sample,
    SampleWithFeatures,
};
use drawing_commons::{
    FEATURES, IMG_DIR, JSON_DIR, MIN_MAX_JS, RAW_DIR, SAMPLES, TESTING, TESTING_FEATURES, TRAINING,
    TRAINING_FEATURES,
};
use std::collections::HashMap;
use std::io::{stdout, ErrorKind, Write};
use std::path::PathBuf;
use termion::clear;
use termion::cursor::Goto;
use termion::raw::IntoRawMode;

type SortedDrawings = Vec<(String, DrawingPaths<[i32; 2]>)>;

pub struct SortedDrawingData {
    session: u64,
    student: String,
    drawings: SortedDrawings,
}

fn file_to_drawing_data(file_name: &PathBuf) -> Result<SortedDrawingData, std::io::Error> {
    let result = std::fs::read_to_string(file_name).and_then(|content| {
        serde_json::from_str::<DrawingData>(content.as_str())
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
    })?;

    let DrawingData {
        session,
        student,
        drawings,
    } = result;

    let mut drawings = drawings
        .into_iter()
        .filter(|x| !x.1.is_empty())
        .collect::<Vec<_>>();
    drawings.sort_by(|a, b| a.0.cmp(&b.0));

    let result = SortedDrawingData {
        session,
        student,
        drawings,
    };

    Ok(result)
}

pub fn read_drawing_data() -> Result<Vec<SortedDrawingData>, std::io::Error> {
    Ok(std::fs::read_dir(RAW_DIR)?
        .flat_map(|x| x)
        .map(|res| res.path())
        .flat_map(|file_name| file_to_drawing_data(&file_name))
        .collect())
}

fn get_samples(inputs: &Vec<SortedDrawingData>) -> Vec<Sample> {
    inputs
        .iter()
        .flat_map(|record| {
            record
                .drawings
                .iter()
                .map(|(label, _)| (label.to_owned(), record.student.to_owned(), record.session))
        })
        .zip(1..)
        .map(|((label, student_name, student_id), id)| Sample {
            id,
            label,
            student_name,
            student_id,
        })
        .collect()
}

pub fn store_samples(inputs: &Vec<SortedDrawingData>) -> Result<(), std::io::Error> {
    let samples = get_samples(&inputs);

    let json = serde_json::to_string(&samples)
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

    std::fs::write(SAMPLES, json)
}

pub fn get_drawings_by_id(inputs: &Vec<SortedDrawingData>) -> HashMap<u64, Vec<Vec<[i32; 2]>>> {
    inputs
        .iter()
        .flat_map(|record| record.drawings.iter().map(|(_, drawings)| drawings.clone()))
        .zip(1..)
        .map(|(drawings, id)| (id, drawings))
        .collect()
}

#[allow(dead_code)]
pub fn store_drawings_as_json(
    drawings: &HashMap<u64, Vec<Vec<[i32; 2]>>>,
) -> Result<(), std::io::Error> {
    for ((id, draw), count) in drawings.iter().zip(1..) {
        let json = serde_json::to_string(&draw)
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

        let path = std::format!("{}/{}.json", JSON_DIR, id);
        print_progress("Generating jsons", count, drawings.len());
        std::fs::write(path, json)?;
    }

    Ok(())
}

fn print_progress(label: &str, count: usize, max: usize) {
    let progress = std::format!(
        "{label} progress: {}/{} ({:.1}%)",
        count,
        max,
        count as f32 * 100.0 / max as f32
    );

    if let Some(mut stdout) = stdout().into_raw_mode().ok() {
        write!(stdout, "{}{}", clear::CurrentLine, Goto(1, 1)).unwrap();
        write!(stdout, "{}", progress).unwrap();
        stdout.flush().unwrap();

        write!(stdout, "{}", termion::cursor::Show).unwrap();
    } else {
        println!("{}", progress);
    }
}

pub fn store_drawings_as_png(drawings: &HashMap<u64, Vec<Vec<[i32; 2]>>>) {
    for (drawing, count) in drawings.iter().zip(1..) {
        let path = std::format!("{}/{}.png", IMG_DIR, drawing.0);
        print_progress("Generating images", count, drawings.len());
        generate_image_file(path.as_str(), drawing.1);
    }
}

fn build_features_for(
    samples: &[Sample],
    min_max: Option<(Vec<f64>, Vec<f64>)>,
) -> (FeaturesData, Vec<f64>, Vec<f64>) {
    let points = samples
        .iter()
        .map(|sample| {
            let path = std::format!("{}/{}.json", JSON_DIR, sample.id);

            let draw_paths = std::fs::read_to_string(path)
                .and_then(|content| {
                    serde_json::from_str::<DrawingPaths<[i32; 2]>>(content.as_str())
                        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
                })
                .expect("");

            let feature = draw_paths.get_feature(|x| x[0] as f64, |x| x[1] as f64);
            vec![feature.x, feature.y]
        })
        .collect::<Vec<_>>();

    let ((min, max), points) = match min_max {
        Some((min, max)) => {
            let points = normalize_points(&min, &max, points);
            ((min, max), points)
        }
        None => normalize_points_to_min_max(points),
    };

    let features = samples
        .iter()
        .zip(points.into_iter())
        .map(|(sample, points)| SampleWithFeatures::create(sample.clone(), points))
        .collect::<Vec<_>>();

    let feature_names = get_feature_names()
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

    (
        FeaturesData {
            feature_names,
            features,
        },
        min,
        max,
    )
}

fn save_features(
    features: &FeaturesData,
    file_name: &str,
    min_max: Option<(Vec<f64>, Vec<f64>)>,
) -> Result<(), std::io::Error> {
    let features_json = serde_json::to_string(&features)
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

    std::fs::write(file_name, features_json)?;

    if let Some(min_max) = min_max {
        let min_max_json = serde_json::to_string(&vec![min_max.0, min_max.1])
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

        std::fs::write(MIN_MAX_JS, min_max_json)?
    }

    Ok(())
}

pub fn build_features() -> Result<(), std::io::Error> {
    println!("EXTRACTING FEATURES...");

    let samples = std::fs::read_to_string(SAMPLES).and_then(|content| {
        serde_json::from_str::<Vec<Sample>>(content.as_str())
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
    })?;

    let training_amount = samples.len() / 2;

    let (features, _, _) = build_features_for(&samples, None);
    save_features(&features, FEATURES, None)?;

    println!("EXTRACTING SPLITS...");
    let (training, testing) = samples.split_at(training_amount);

    let min_max = {
        let (features, min, max) = build_features_for(training, None);
        let features_json = serde_json::to_string(&training)
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

        std::fs::write(TRAINING, features_json)?;
        save_features(
            &features,
            TRAINING_FEATURES,
            Some((min.clone(), max.clone())),
        )?;
        (min, max)
    };

    {
        let (features, _, _) = build_features_for(testing, Some(min_max));
        let features_json = serde_json::to_string(&testing)
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

        std::fs::write(TESTING, features_json)?;
        save_features(&features, TESTING_FEATURES, None)?;
    }

    Ok(())
}
