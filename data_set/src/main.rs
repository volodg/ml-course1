mod file_utils;

use crate::file_utils::{get_samples, read_drawing_data, SAMPLES};
use std::io::ErrorKind;

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    let samples = get_samples(&drawing_data);

    let json = serde_json::to_string(&samples)
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

    std::fs::write(SAMPLES, json)?;

    println!("Valid entries count {:?}", samples.len());

    Ok(())
}
