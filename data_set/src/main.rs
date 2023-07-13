mod file_utils;

use crate::file_utils::{read_drawing_data, store_drawings, store_samples};

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    store_samples(&drawing_data)?;
    store_drawings(&drawing_data)?;

    Ok(())
}
