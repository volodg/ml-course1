use crate::html::HtmlDom;

pub struct AppState {
    pub html: HtmlDom,
    pub point: [f64; 2],
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        let point = [90.0, 120.0];
        Self { html, point }
    }
}
