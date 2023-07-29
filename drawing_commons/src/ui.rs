
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COLOR_PER_LABEL: HashMap<&'static str, &'static str> = {
        let mut result = HashMap::new();

        result.insert("car", "gray",);
        result.insert("fish", "red",);
        result.insert("house", "yellow",);
        result.insert("tree", "green",);
        result.insert("bicycle", "cyan",);
        result.insert("guitar", "blue",);
        result.insert("pencil", "magenta",);
        result.insert("clock", "lightgray",);
        result.insert("?", "red",);

        result
    };
}
