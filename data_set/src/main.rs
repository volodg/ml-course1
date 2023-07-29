extern crate core;

mod draw;
mod file_utils;
mod analytics;

use crate::file_utils::build_features;

fn main() -> Result<(), std::io::Error> {
    build_features()
    // run_evaluations()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
