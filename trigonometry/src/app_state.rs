use crate::html::HtmlDom;

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
}
