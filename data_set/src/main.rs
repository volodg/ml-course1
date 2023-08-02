extern crate core;

use crate::analytics::custom_knn::run_knn_evaluations;
use crate::file_utils::{build_data_set, build_features};

mod analytics;
mod draw;
mod file_utils;

fn main() -> Result<(), std::io::Error> {
    build_data_set()?;
    build_features()?;
    run_knn_evaluations()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
