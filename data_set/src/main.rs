use std::io::ErrorKind;
use std::path::PathBuf;
use const_format::concatcp;
use drawing_commons::DrawingData;
use serde_json;

const DATA_DIR: &str = "./data";
const RAW_DIR: &str = concatcp!(DATA_DIR, "/raw");
#[allow(dead_code)]
const DATASET_DIR: &str = concatcp!(DATA_DIR, "/dataset");
#[allow(dead_code)]
const JSON_DIR: &str = concatcp!(DATA_DIR, "/json");
#[allow(dead_code)]
const IMG_DIR: &str = concatcp!(DATA_DIR, "/img");
#[allow(dead_code)]
const SAMPLES: &str = concatcp!(DATA_DIR, "/samples.json");

fn file_to_drawing_data(file_name: &PathBuf) -> Result<DrawingData, std::io::Error> {
    std::fs::read_to_string(file_name).and_then(|content| {
        serde_json::from_str::<DrawingData>(content.as_str()).map_err(|err|{
            std::io::Error::new(ErrorKind::InvalidData, err)
        })
    })
}

struct Sample {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    label: String,
    #[allow(dead_code)]
    student_name: String,
    #[allow(dead_code)]
    student_id: u64,
}

fn main() -> Result<(), std::io::Error> {
    let entries: Vec<_> = std::fs::read_dir(RAW_DIR)?
        .flat_map(|x| x)
        .map(|res| res.path())
        .flat_map(|file_name| {
            file_to_drawing_data(&file_name)
        })
        .collect();

    let samples: Vec<_> = entries
        .iter()
        .zip(1..)
        .flat_map(|(record, id)| {
            record.get_drawings().iter().map(move |(label, _)| {
                Sample {
                    id,
                    label: label.to_owned(),
                    student_name: record.get_student().to_owned(),
                    student_id: record.get_session(),
                }
            })
        }).collect();

    println!("Valid entries count {:?}", samples.len());

    Ok(())
}
