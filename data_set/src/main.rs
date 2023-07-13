extern crate core;

mod file_utils;
mod draw;

use crate::draw::generate_image_file;
use crate::file_utils::{get_drawings_by_id, read_drawing_data};

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    // store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);
    // store_drawings_as_json(&drawings)?;

    generate_image_file("", &drawings[&1]);

    Ok(())
}
