use std::collections::HashMap;
use web_sys::Element;

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Sample {
    pub id: i32,
    pub label: String,
    pub point: Point,
}

impl Sample {
    pub fn create(id: i32, label: String, point: Point) -> Self {
        Self { id, label, point }
    }
}

pub struct Options {
    pub size: u32,
    pub axis_labels: [String; 2],
    pub styles: HashMap<String, String>,
}

pub struct Chart {}

impl Chart {
    pub fn create(_element: Element, _samples: Vec<Sample>, _options: Options) -> Self {
        Self {}
    }
}
