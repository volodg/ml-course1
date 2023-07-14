extern crate core;

mod draw;
mod file_utils;

use std::io::ErrorKind;
use drawing_commons::{JSON_DIR, SAMPLES};
use drawing_commons::models::Sample;
use crate::file_utils::{get_drawings_by_id, Paths, read_drawing_data, store_drawings_as_json, store_drawings_as_png, store_samples};

#[allow(dead_code)]
fn build_data_set() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);
    store_drawings_as_json(&drawings)?;
    store_drawings_as_png(&drawings);

    Ok(())
}

fn build_features() -> Result<(), std::io::Error> {
    let samples = std::fs::read_to_string(SAMPLES).and_then(|content| {
        serde_json::from_str::<Vec<Sample>>(content.as_str())
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
    })?;

    for sample in samples {
        let path = std::format!("{}/{}.json", JSON_DIR, sample.id);

        let _draw_paths = std::fs::read_to_string(path).and_then(|content| {
            serde_json::from_str::<Paths>(content.as_str())
                .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
        })?;
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    build_features()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
