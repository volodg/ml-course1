use std::fmt;
use crate::html::HtmlDom;
use commons::math::remap;
use rand::Rng;
use web_commons::chart::{Point, Sample};

#[derive(PartialEq)]
pub enum CarType {
    Basic,
    Sport
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
    pub id: i32,
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
            value.id, value.car_type.to_string(), Point {
                x: value.km,
                y: value.price,
            }
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
                let km = rng.gen_range(3000.0..300000.0);
                let price = remap(3000.0, 300000.0, 9000.0, 900.0, km)
                    + rng.gen_range(-2000.0..2000.0)
                    + if car_type == CarType::Basic { 0.0 } else { 5000.0 };

                Car { id, car_type, km, price, }.into()
            })
            .collect::<Vec<_>>();

        Self { html, samples }
    }
}
