use raqote::{DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};
use crate::file_utils::Paths;

const STROKE_STYLE: StrokeStyle = StrokeStyle {
    cap: LineCap::Round,
    join: LineJoin::Round,
    width: 3.,
    miter_limit: 10.,
    dash_array: vec![],
    dash_offset: 0.,
};

const LINE_SOURCE: Source = Source::Solid(SolidSource {
    r: 0x0,
    g: 0x0,
    b: 0x0,
    a: 255,
});

pub fn generate_image_file(file: &str, paths: &Paths) {
    let mut dt = DrawTarget::new(400, 400);

    dt.clear(SolidSource {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });

    for path in paths {
        let mut pb = PathBuilder::new();

        pb.move_to(path[0][0] as f32, path[0][1] as f32);

        for point in &path[1..] {
            pb.line_to(point[0] as f32, point[1] as f32);
        }

        let path = pb.finish();

        dt.stroke(
            &path,
            &LINE_SOURCE,
            &STROKE_STYLE,
            &DrawOptions::new(),
        );
    }

    dt.write_png(file).unwrap()
}
