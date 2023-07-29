extern crate core;

mod draw;
mod file_utils;
mod analytics;

use crate::analytics::knn::knn;
use crate::file_utils::build_features;

fn main() -> Result<(), std::io::Error> {
    knn();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
