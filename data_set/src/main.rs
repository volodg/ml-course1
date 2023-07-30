extern crate core;

use crate::analytics::custom_knn::run_knn_evaluations;

mod analytics;
mod draw;
mod file_utils;

fn main() -> Result<(), std::io::Error> {
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
