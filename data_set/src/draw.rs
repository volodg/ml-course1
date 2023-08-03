use drawing_commons::draw_images::DrawTargetExt;
use drawing_commons::models::{DrawingPaths, Features};
use raqote::DrawTarget;

pub fn generate_image_file(file: &str, paths: &DrawingPaths<[f64; 2]>) {
    let mut dt = DrawTarget::new(400, 400);

    dt.draw_path(paths, 3.0);

    let pixels = paths.get_pixels(true);
    for index in 0..pixels.len() {
        let alpha = pixels[index];

        dt.get_data_mut()[index] = (0 as u32) << 16;
        dt.get_data_mut()[index] = (0 as u32) << 8;
        dt.get_data_mut()[index] = (0 as u32);
        dt.get_data_mut()[index] = (alpha as u32) << 24;
    }

    dt.write_png(file).unwrap()
}
