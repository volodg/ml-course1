use commons::geometry::{graham_scan, minimum_bounding_box, polygon_roundness};
use drawing_commons::models::DrawingPaths;
use raqote::DrawTarget;
use drawing_commons::draw_images::DrawTargetExt;

pub fn generate_image_file(file: &str, paths: &DrawingPaths<[f64; 2]>) {
    let mut dt = DrawTarget::new(400, 400);

    dt.draw_path(3.0, paths);

    let all_points = paths.clone().into_iter().flatten().collect::<Vec<_>>();
    let mut hull = graham_scan(&all_points);
    let (vertices, _, _) = minimum_bounding_box(&hull).expect("");

    let red = 255;
    let green = 0;
    let blue = 0;

    let paths = vec![
        vertices[0],
        vertices[1],
        vertices[2],
        vertices[3],
        vertices[0],
    ];
    dt.draw_path_with_color(10.0, &vec![paths], (red, green, blue, 255));

    let red = 0;
    let green = 0;
    let blue = 255;

    hull.push(hull[0]);
    dt.draw_path_with_color(10.0, &vec![hull.clone()], (red, green, blue, 255));

    let roundness = polygon_roundness(&hull);

    let red = (255.0 - (255.0 * roundness.powi(5)).floor()) as u8;
    let green = 255;
    let blue = (255.0 - (255.0 * (1.0 - roundness.powi(5))).floor()) as u8;

    hull.push(hull[0]);
    dt.draw_path_with_color(10.0, &vec![hull], (red, green, blue, 255));

    dt.write_png(file).unwrap()
}
