extern crate core;

mod file_utils;
mod draw;

use crate::draw::generate_image_file;
use crate::file_utils::{get_drawings_by_id, IMG_DIR, read_drawing_data};

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    // store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);
    // store_drawings_as_json(&drawings)?;

    for drawing in &drawings {
        let path = std::format!("{}/{}.png", IMG_DIR, drawing.0);
        generate_image_file(path.as_str(), drawing.1);
    }

    Ok(())
}
