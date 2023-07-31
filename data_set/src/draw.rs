use commons::geometry::{graham_scan, polygon_roundness};
use drawing_commons::models::DrawingPaths;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};

const STROKE_STYLE: StrokeStyle = StrokeStyle {
    cap: LineCap::Round,
    join: LineJoin::Round,
    width: 3.,
    miter_limit: 10.,
    dash_array: vec![],
    dash_offset: 0.,
};

trait DrawTargetExt {
    fn draw_path(&mut self, paths: &DrawingPaths<[i32; 2]>);
    fn draw_path_with_color(&mut self, paths: &DrawingPaths<[i32; 2]>, color: (u8, u8, u8, u8));
}

impl DrawTargetExt for DrawTarget {
    fn draw_path(&mut self, paths: &DrawingPaths<[i32; 2]>) {
        self.draw_path_with_color(paths, (0, 0, 0, 255))
    }

    fn draw_path_with_color(&mut self, paths: &DrawingPaths<[i32; 2]>, color: (u8, u8, u8, u8)) {
        for path in paths {
            let mut pb = PathBuilder::new();

            pb.move_to(path[0][0] as f32, path[0][1] as f32);

            for point in &path[1..] {
                pb.line_to(point[0] as f32, point[1] as f32);
            }

            let path = pb.finish();

            let source = Source::Solid(SolidSource {
                r: color.0,
                g: color.1,
                b: color.2,
                a: color.3,
            });

            self.stroke(&path, &source, &STROKE_STYLE, &DrawOptions::new());
        }
    }
}

pub fn generate_image_file(file: &str, paths: &DrawingPaths<[i32; 2]>) {
    let mut dt = DrawTarget::new(400, 400);

    dt.draw_path(paths);

    let all_points = paths.clone().into_iter().flatten().collect::<Vec<_>>();
    let hull = graham_scan(&all_points);
    let roundness =
        polygon_roundness(&hull.clone().into_iter().map(|x| vec![x[0], x[1]]).collect());

    let red = (255.0 * roundness).floor() as u8;
    let green = 0;
    let blue = (255.0 * (1.0 - roundness)).floor() as u8;

    dt.draw_path_with_color(&vec![hull], (red, green, blue, 255));

    dt.write_png(file).unwrap()
}
