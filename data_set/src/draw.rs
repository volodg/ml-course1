use drawing_commons::draw_images::DrawTargetExt;
use drawing_commons::models::{DrawingPaths, Features};
use raqote::DrawTarget;

pub fn generate_image_file(file: &str, paths: &DrawingPaths<[f64; 2]>) {
    let mut dt = DrawTarget::new(400, 400);

    dt.draw_path(paths, 3.0);

    let complexity = paths.get_pixels(true).into_iter().filter(|x| *x != 0).count();
    dt.draw_text_simplified(std::format!("{}", complexity).as_str());

    dt.write_png(file).unwrap()
}
