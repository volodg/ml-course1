extern crate core;

use crate::file_utils::{build_data_set, build_features};

mod analytics;
mod draw;
mod file_utils;

fn main() -> Result<(), std::io::Error> {
    build_data_set()?;
    build_features()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
