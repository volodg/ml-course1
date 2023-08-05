use commons::math::lerp::lerp;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct Road {
    context: CanvasRenderingContext2d,
    #[allow(dead_code)]
    x: f64,
    #[allow(dead_code)]
    width: f64,
    #[allow(dead_code)]
    lane_count: usize,
    #[allow(dead_code)]
    left: f64,
    #[allow(dead_code)]
    right: f64,
    #[allow(dead_code)]
    top: f64,
    #[allow(dead_code)]
    bottom: f64,
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

        Self {
            context,
            x,
            width,
            lane_count,
            left,
            right,
            top: -infinity,
            bottom: infinity,
        }
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.set_line_width(5.0);
        self.context.set_stroke_style(&JsValue::from_str("white"));

        for i in 0..=self.lane_count {
            let x = lerp(self.left, self.right, i as f64 / self.lane_count as f64);

            if i > 0 && i < self.lane_count {
                let array = Array::of2(&JsValue::from(20.0), &JsValue::from(20.0));
                self.context.set_line_dash(&array)?;
            } else {
                self.context.set_line_dash(&Array::new())?;
            }

            self.context.begin_path();
            self.context.move_to(x, self.top);
            self.context.line_to(x, self.bottom);
            self.context.stroke();
        }

        Ok(())
    }

    pub fn get_lane_center(&self, index: usize) -> f64 {
        let lane_width = self.width / self.lane_count as f64;
        self.left + lane_width / 2.0 + index.min(self.lane_count - 1) as f64 * lane_width
    }
}
