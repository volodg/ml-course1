mod file_utils;

use image::{ImageBuffer, Rgba};
use itertools::Itertools;
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use crate::file_utils::{get_drawings_by_id, Paths, read_drawing_data};

// Try
// https://github.com/jrmuizel/raqote

fn generate_image_file(_file: &str, paths: &Paths) {
    let width = 400;
    let height = 400;
    let mut img = ImageBuffer::new(width, height);

    // White background
    let rect = Rect::at(0, 0).of_size(400, 400);
    let color = Rgba([255u8, 255u8, 255u8, 255u8]);
    draw_filled_rect_mut(&mut img, rect, color);

    for path in paths {
        for (point1, point2) in path.iter().tuple_windows() {
            // Draw line
            let start = (point1[0] as f32, point1[1] as f32);
            let end = (point2[0] as f32, point2[1] as f32);
            let line_color = Rgba([0u8, 0u8, 0u8, 255u8]);
            draw_line_segment_mut(&mut img, start, end, line_color);
        }
    }

    img.save("output.png").unwrap();
}

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    // store_samples(&drawing_data)?;

    let drawings = get_drawings_by_id(&drawing_data);
    // store_drawings_as_json(&drawings)?;

    generate_image_file("", &drawings[&1]);

    Ok(())
}
