use crate::models::DrawingPaths;
use commons::geometry::Point2DView;
use itertools::Itertools;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait CanvasRenderingContext2dExt {
    fn draw_path<T: Point2DView>(&self, paths: &DrawingPaths<T>, width: f32);
}

impl CanvasRenderingContext2dExt for CanvasRenderingContext2d {
    fn draw_path<T: Point2DView>(&self, paths: &DrawingPaths<T>, width: f32) {
        for path in paths {
            if path.is_empty() {
                continue;
            }

            for (from, to) in path.iter().tuple_windows() {
                self.begin_path();
                self.set_line_width(width.into());
                self.set_stroke_style(&JsValue::from_str("white"));
                self.set_line_cap("round");
                self.set_line_join("round");

                self.move_to(from.x() as f64, from.y() as f64);
                self.line_to(to.x() as f64, to.y() as f64);

                self.stroke();
            }
        }
    }
}
