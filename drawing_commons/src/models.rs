use commons::math::{min_max, Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Sample {
    pub id: usize,
    pub label: String,
    pub student_name: String,
    pub student_id: u64,
}

type Drawings = HashMap<String, Vec<Vec<[i32; 2]>>>;

#[derive(Deserialize, Serialize)]
pub struct DrawingData {
    pub session: u64,
    pub student: String,
    pub drawings: Drawings,
}

impl DrawingData {
    pub fn create(session: u64, student: String, drawings: Drawings) -> Self {
        Self {
            session,
            student,
            drawings,
        }
    }

    pub fn get_student(&self) -> &String {
        &self.student
    }

    pub fn get_session(&self) -> u64 {
        self.session
    }

    pub fn get_drawings(&self) -> &Drawings {
        &self.drawings
    }
}

pub type DrawingPaths<T> = Vec<Vec<T>>;

pub trait Features {
    type ElType;

    fn path_count(&self) -> usize;

    fn point_count(&self) -> usize;

    fn get_width(&self, el_getter: impl Fn(&Self::ElType) -> f64) -> usize;

    fn get_feature(
        &self,
        x_getter: impl Fn(&Self::ElType) -> f64,
        y_getter: impl Fn(&Self::ElType) -> f64,
    ) -> Point;
}

impl<T> Features for DrawingPaths<T> {
    type ElType = T;

    fn path_count(&self) -> usize {
        self.len()
    }

    fn point_count(&self) -> usize {
        self.iter().fold(0, |acc, drawing| acc + drawing.len())
    }

    fn get_width(&self, el_getter: impl Fn(&Self::ElType) -> f64) -> usize {
        let zero_min_max: Option<f64> = None;
        let (min_x, max_x) =
            self.iter()
                .flatten()
                .fold((zero_min_max, zero_min_max), |(min_x, max_x), el| {
                    let x_minmax = min_max((min_x, max_x), el_getter(el));
                    (Some(x_minmax.0), Some(x_minmax.1))
                });

        match (max_x, min_x) {
            (Some(max_x), Some(min_x)) => (max_x.round() as i32 - min_x.round() as i32) as usize,
            (_, _) => 0,
        }
    }

    fn get_feature(
        &self,
        _x_getter: impl Fn(&Self::ElType) -> f64,
        _y_getter: impl Fn(&Self::ElType) -> f64,
    ) -> Point {
        Point {
            // x: self.get_width(x_getter) as f64,
            // y: self.get_width(y_getter) as f64,
            x: self.path_count() as f64,
            y: self.point_count() as f64,
        }
    }
}

pub fn get_feature_names() -> [String; 2] {
    ["Path Count".to_owned(), "Point Count".to_owned()]
    // ["Width".to_owned(), "Height".to_owned()]
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SampleWithFeatures {
    pub label: String,
    pub point: Vec<f64>,
}

impl SampleWithFeatures {
    pub fn create(label: String, point: Vec<f64>) -> Self {
        Self { label, point }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesData {
    pub feature_names: Vec<String>,
    pub features: Vec<SampleWithFeatures>,
}
