use commons::geometry::{Line2D, Point2D, Point2DView};
use commons::math::lerp::lerp;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct Road {
    context: CanvasRenderingContext2d,
    #[allow(dead_code)]
    x: f64,
    width: f64,
    lane_count: usize,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    borders: Vec<Line2D>,
}

impl Road {
    pub fn create(context: CanvasRenderingContext2d, x: f64, width: f64) -> Self {
        Self::create_with_lane_count(context, x, width, 3)
    }

    pub fn create_with_lane_count(
        context: CanvasRenderingContext2d,
        x: f64,
        width: f64,
        lane_count: usize,
    ) -> Self {
        let left = x - width / 2.0;
        let right = x + width / 2.0;
        let infinity = 1_000_000.0;

        let top = -infinity;
        let bottom = infinity;

        let top_left = Point2D::create(left, top);
        let top_right = Point2D::create(right, top);
        let bottom_left = Point2D::create(left, bottom);
        let bottom_right = Point2D::create(right, bottom);

        Self {
            context,
            x,
            width,
            lane_count,
            left,
            right,
            top,
            bottom,
            borders: vec![
                Line2D {
                    start: top_left,
                    end: bottom_left,
                },
                Line2D {
                    start: top_right,
                    end: bottom_right,
                },
            ],
        }
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.set_line_width(5.0);
        self.context.set_stroke_style(&JsValue::from_str("white"));

        for i in 1..self.lane_count {
            let x = lerp(self.left, self.right, i as f64 / self.lane_count as f64);

            let array = Array::of2(&JsValue::from(20.0), &JsValue::from(20.0));
            self.context.set_line_dash(&array)?;

            self.context.begin_path();
            self.context.move_to(x, self.top);
            self.context.line_to(x, self.bottom);
            self.context.stroke();
        }

        self.context.set_line_dash(&Array::new())?;

        for border in &self.borders {
            self.context.begin_path();
            self.context.move_to(border.start.x, border.start.y);
            self.context.line_to(border.end.x, border.end.y);
            self.context.stroke();
        }

        Ok(())
    }

    pub fn get_lane_center(&self, index: usize) -> f64 {
        let lane_width = self.width / self.lane_count as f64;
        self.left + lane_width / 2.0 + index.min(self.lane_count - 1) as f64 * lane_width
    }
}
