use std::path::PathBuf;
use const_format::concatcp;

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

fn main() -> Result<(), std::io::Error> {
    let entries: Vec<PathBuf> = std::fs::read_dir(RAW_DIR)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<_, _>>()?;

    println!("Hello, world! {:?}", entries);
    Ok(())
}
