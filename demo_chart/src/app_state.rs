use crate::html::HtmlDom;
use rand::Rng;
use commons::math::remap;

pub struct Car {
    pub id: i32,
    pub label: String,
    pub km: f64,
    pub price: f64,
}

pub struct AppState {
    pub html: HtmlDom,
    pub samples: Vec<Car>,
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        let mut rng = rand::thread_rng();

        let samples = (0..1000)
            .map(|id| {
                let type_ = if rng.gen_range(0.0..1.0) < 0.5 {
                    "basic"
                } else {
                    "sport"
                };
                let km = rng.gen_range(3000.0..300000.0);
                let price = remap(3000.0, 300000.0, 9000.0, 900.0, km);
                Car {
                    id,
                    label: type_.to_owned(),
                    km,
                    price,
                }
            })
            .collect::<Vec<_>>();

        Self { html, samples }
    }
}
