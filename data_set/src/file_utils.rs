use const_format::concatcp;
use drawing_commons::DrawingData;
use serde::Serialize;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;

const DATA_DIR: &str = "./data";
const RAW_DIR: &str = concatcp!(DATA_DIR, "/raw");
const DATASET_DIR: &str = concatcp!(DATA_DIR, "/dataset");
#[allow(dead_code)]
const JSON_DIR: &str = concatcp!(DATASET_DIR, "/json");
#[allow(dead_code)]
const IMG_DIR: &str = concatcp!(DATASET_DIR, "/img");
#[allow(dead_code)]
const SAMPLES: &str = concatcp!(DATASET_DIR, "/samples.json");

pub type Paths = Vec<Vec<[i32; 2]>>;
type SortedDrawings = Vec<(String, Paths)>;

pub struct SortedDrawingData {
    #[allow(dead_code)]
    session: u64,
    #[allow(dead_code)]
    student: String,
    #[allow(dead_code)]
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

    let mut drawings = drawings.into_iter().filter(|x| !x.1.is_empty()).collect::<Vec<_>>();
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

#[derive(Serialize)]
pub struct Sample {
    id: usize,
    label: String,
    student_name: String,
    student_id: u64,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    for (id, drawings) in drawings {
        let json = serde_json::to_string(&drawings)
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

        let path = std::format!("{}/{}.json", JSON_DIR, id);
        std::fs::write(path, json)?;
    }

    Ok(())
}
