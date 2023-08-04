use web_sys::CanvasRenderingContext2d;

pub struct Car {
    #[allow(dead_code)]
    context: CanvasRenderingContext2d,
    #[allow(dead_code)]
    x: f64,
    #[allow(dead_code)]
    y: f64,
    #[allow(dead_code)]
    width: f64,
    #[allow(dead_code)]
    height: f64
}

impl Car {
    pub fn create(
        context: CanvasRenderingContext2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64) -> Self {
        Self { context, x, y, width, height }
    }

    pub fn draw(&self) {
        // self.con
    }
}
