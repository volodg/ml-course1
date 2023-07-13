use const_format::concatcp;
use drawing_commons::DrawingData;
use serde::Serialize;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;

const DATA_DIR: &str = "./data";
const RAW_DIR: &str = concatcp!(DATA_DIR, "/raw");
#[allow(dead_code)]
const DATASET_DIR: &str = concatcp!(DATA_DIR, "/dataset");
#[allow(dead_code)]
const JSON_DIR: &str = concatcp!(DATA_DIR, "/json");
#[allow(dead_code)]
const IMG_DIR: &str = concatcp!(DATA_DIR, "/img");
pub const SAMPLES: &str = concatcp!(DATA_DIR, "/samples.json");

fn file_to_drawing_data(file_name: &PathBuf) -> Result<DrawingData, std::io::Error> {
    std::fs::read_to_string(file_name).and_then(|content| {
        serde_json::from_str::<DrawingData>(content.as_str())
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
    })
}

pub fn read_drawing_data() -> Result<Vec<DrawingData>, std::io::Error> {
    Ok(std::fs::read_dir(RAW_DIR)?
        .flat_map(|x| x)
        .map(|res| res.path())
        .flat_map(|file_name| file_to_drawing_data(&file_name))
        .collect())
}

#[derive(Serialize)]
pub struct Sample {
    id: usize,
    label: String,
    student_name: String,
    student_id: u64,
}

pub fn get_samples(inputs: &Vec<DrawingData>) -> Vec<Sample> {
    inputs
        .iter()
        .flat_map(|record| {
            record.get_drawings().iter().map(|(label, _)| {
                (
                    label.to_owned(),
                    record.get_student().to_owned(),
                    record.get_session(),
                )
            })
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

pub fn get_drawings_by_id(inputs: &Vec<DrawingData>) -> HashMap<u64, Vec<Vec<[i32; 2]>>> {
    inputs
        .iter()
        .flat_map(|record| {
            record
                .get_drawings()
                .iter()
                .map(|(_, drawings)| drawings.clone())
        })
        .zip(1..)
        .map(|(drawings, id)| (id, drawings))
        .collect()
}
