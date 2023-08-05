
pub struct Road {
    x: f64,
    width: f64,
    lane_count: usize,
    left: f64,
    right: f64,
    infinity: f64,
}

impl Road {
    fn create(x: f64, width: f64, lane_count: usize,) -> Self {
        Self::create_with_lane_count(x, width, 3)
    }

    fn create_with_lane_count(x: f64, width: f64, lane_count: usize,) -> Self {
        let left = x - width / 2.0;
        let right = x - width / 2.0;
        let infinity = 1_000_000.0;

        Self {
            x,
            width,
            lane_count,
            left,
            right,
            infinity,
        }
    }
}
