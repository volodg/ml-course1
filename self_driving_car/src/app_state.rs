use crate::html::HtmlDom;
use commons::network::NeuralNetwork;

pub struct AppState {
    pub html: HtmlDom,
    pub best_car: Option<NeuralNetwork>,
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        Self {
            html,
            best_car: None,
        }
    }
}
