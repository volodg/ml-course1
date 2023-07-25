use crate::html::HtmlDom;
use commons::math::{remap, Point};
use rand::Rng;
use std::fmt;
use web_commons::chart_models::Sample;

#[derive(PartialEq)]
pub enum CarType {
    Basic,
    Sport,
}

impl fmt::Display for CarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CarType::Basic => write!(f, "basic"),
            CarType::Sport => write!(f, "sport"),
        }
    }
}

pub struct Car {
    pub id: usize,
    pub car_type: CarType,
    pub km: f64,
    pub price: f64,
}

pub struct AppState {
    pub html: HtmlDom,
    pub samples: Vec<Sample>,
}

impl From<Car> for Sample {
    fn from(value: Car) -> Self {
        Sample::create(
            value.id,
            value.car_type.to_string(),
            Point {
                x: value.km,
                y: value.price,
            },
        )
    }
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        let mut rng = rand::thread_rng();

        let samples = (0..1000)
            .map(|id| {
                let car_type = if rng.gen_range(0.0..1.0) < 0.5 {
                    CarType::Basic
                } else {
                    CarType::Sport
                };
                let km = rng.gen_range(3_000.0..300_000.0);
                let price = remap(3_000.0, 300_000.0, 9_000.0, 900.0, km)
                    + rng.gen_range(-2_000.0..2_000.0)
                    + if car_type == CarType::Basic {
                        0.0
                    } else {
                        5_000.0
                    };

                Car {
                    id,
                    car_type,
                    km,
                    price,
                }
                .into()
            })
            .collect::<Vec<_>>();

        Self { html, samples }
    }
}
