extern crate core;

mod draw;
mod file_utils;

use crate::file_utils::{
    build_features, get_drawings_by_id, read_drawing_data, store_drawings_as_json,
    store_drawings_as_png, store_samples,
};

#[allow(dead_code)]
fn build_data_set() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);
    store_drawings_as_json(&drawings)?;
    store_drawings_as_png(&drawings);

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
