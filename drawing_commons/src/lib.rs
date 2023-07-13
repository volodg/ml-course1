use const_format::concatcp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DATA_DIR: &str = "./data";
pub const RAW_DIR: &str = concatcp!(DATA_DIR, "/raw");
const DATASET_DIR: &str = concatcp!(DATA_DIR, "/dataset");
pub const JSON_DIR: &str = concatcp!(DATASET_DIR, "/json");
pub const IMG_DIR: &str = concatcp!(DATASET_DIR, "/img");
pub const SAMPLES: &str = concatcp!(DATASET_DIR, "/samples.json");

pub const FLAGGED_USERS: &[u64; 3] = &[1663882102141,1663900040545,1664485938220];

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
