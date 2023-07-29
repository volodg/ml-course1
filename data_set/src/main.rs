extern crate core;

mod analytics;
mod draw;
mod file_utils;

use crate::analytics::knn::knn;

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
