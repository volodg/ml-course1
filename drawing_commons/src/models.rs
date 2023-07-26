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
    fn path_count(&self) -> usize;

    fn point_count(&self) -> usize;
}

impl<T> Features for DrawingPaths<T> {
    fn path_count(&self) -> usize {
        self.len()
    }

    fn point_count(&self) -> usize {
        self.iter().fold(0, |acc, drawing| acc + drawing.len())
    }
}

#[derive(Deserialize, Serialize)]
pub struct SampleWithFeatures {
    pub label: String,
    pub point: [usize; 2],
}

impl SampleWithFeatures {
    pub fn create(sample: Sample, point: [usize; 2]) -> Self {
        Self {
            label: sample.label,
            point,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesData {
    pub feature_names: Vec<String>,
    pub features: Vec<SampleWithFeatures>,
}
