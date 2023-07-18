use crate::html::HtmlDom;

pub struct AppState {
    pub html: HtmlDom,
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        Self {
            html,
        }
    }
}
