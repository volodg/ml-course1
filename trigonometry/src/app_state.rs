use crate::html::HtmlDom;
use commons::geometry::distance;
use js_sys::Math::asin;

pub struct AppState {
    pub html: HtmlDom,

    pub point_a: [i32; 2],
    pub point_b: [i32; 2],
    pub point_c: [i32; 2],
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        let point_a = [0, 0];
        let point_b = [90, 120];
        let point_c = [point_b[0], 0];

        Self {
            html,
            point_a,
            point_b,
            point_c,
        }
    }

    pub fn update_points(&mut self, position: &[i32; 2]) {
        self.point_b[0] = position[0] - self.html.canvas.canvas.offset[0];
        self.point_b[1] = position[1] - self.html.canvas.canvas.offset[1];

        self.point_c[0] = self.point_b[0];
    }

    fn get_dist_a(&self) -> f64 {
        distance(&self.point_b, &self.point_c)
    }

    fn get_dist_b(&self) -> f64 {
        distance(&self.point_c, &self.point_a)
    }

    fn get_dist_c(&self) -> f64 {
        distance(&self.point_a, &self.point_b)
    }

    pub fn get_sin(&self) -> f64 {
        self.get_dist_a() / self.get_dist_c()
    }

    pub fn get_cos(&self) -> f64 {
        self.get_dist_b() / self.get_dist_c()
    }

    pub fn get_tan(&self) -> f64 {
        self.get_dist_a() / self.get_dist_b()
    }

    pub fn get_theta(&self) -> f64 {
        asin(self.get_sin())
    }
}
