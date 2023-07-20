use crate::html::HtmlDom;
use crate::vector::VectorXY;

pub struct AppState {
    pub html: HtmlDom,
    pub point: VectorXY,
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        let point = VectorXY::new(90.0, 120.0);
        Self { html, point }
    }
}
