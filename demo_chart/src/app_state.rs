use crate::html::HtmlDom;
use rand::Rng;

pub struct Car {
    pub id: i32,
    #[allow(dead_code)]
    pub label: String,
    #[allow(dead_code)]
    pub km: i32,
    #[allow(dead_code)]
    pub price: i32,
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
                let km = rng.gen_range(3000..300000);
                let price = rng.gen_range(900..9000);
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
