use crate::models::DrawingPaths;
use commons::geometry::Point2DView;
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

pub trait DrawTargetExt {
    fn draw_path<T: Point2DView>(&mut self, width: f32, paths: &DrawingPaths<T>);
    fn draw_path_with_color<T: Point2DView>(
        &mut self,
        width: f32,
        paths: &DrawingPaths<T>,
        color: (u8, u8, u8, u8),
    );
}

impl DrawTargetExt for DrawTarget {
    fn draw_path<T: Point2DView>(&mut self, width: f32, paths: &DrawingPaths<T>) {
        self.draw_path_with_color(width, paths, (0, 0, 0, 255))
    }

    fn draw_path_with_color<T: Point2DView>(
        &mut self,
        width: f32,
        paths: &DrawingPaths<T>,
        color: (u8, u8, u8, u8),
    ) {
        for path in paths {
            let mut pb = PathBuilder::new();

            pb.move_to(path[0].x() as f32, path[0].y() as f32);

            for point in &path[1..] {
                pb.line_to(point.x() as f32, point.y() as f32);
            }

            let path = pb.finish();

            let source = Source::Solid(SolidSource {
                r: color.0,
                g: color.1,
                b: color.2,
                a: color.3,
            });

            let mut style = STROKE_STYLE.clone();
            style.width = width;

            self.stroke(&path, &source, &STROKE_STYLE, &DrawOptions::new());
        }
    }
}
