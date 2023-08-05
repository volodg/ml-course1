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

    pub fn create_with_lane_count(context: CanvasRenderingContext2d, x: f64, width: f64, lane_count: usize,) -> Self {
        let left = x - width / 2.0;
        let right = x - width / 2.0;
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

    pub fn draw(&self) {
        self.context.set_line_width(5.0);
    }
}
