mod file_utils;

use crate::file_utils::{get_drawings_by_id, read_drawing_data, SAMPLES, store_samples};
use std::io::ErrorKind;

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);

    let json = serde_json::to_string(&drawings)
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;

    std::fs::write(SAMPLES, json)?;

    println!("samples count {:?}", samples.len());
    println!("drawings count {:?}", drawings.len());

    Ok(())
}
