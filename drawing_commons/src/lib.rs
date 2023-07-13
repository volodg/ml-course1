use serde::Serialize;
use std::collections::HashMap;

type Drawings = HashMap<String, Vec<Vec<[i32; 2]>>>;

#[derive(Serialize)]
pub struct DrawingData {
    session: String,
    student: String,
    drawings: Drawings,
}

impl DrawingData {
    pub fn create(session: String, student: String, drawings: Drawings) -> Self {
        Self {
            session,
            student,
            drawings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
