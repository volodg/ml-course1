use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COLOR_PER_LABEL: HashMap<&'static str, (&'static str, (u8, u8, u8))> = {
        let mut result = HashMap::new();

        result.insert("car", ("gray", (128, 128, 128)));
        result.insert("fish", ("red", (255, 0, 0)));
        result.insert("house", ("yellow", (255, 255, 0)));
        result.insert("tree", ("green", (0, 128, 0)));
        result.insert("bicycle", ("cyan", (0, 255, 255)));
        result.insert("guitar", ("blue", (0, 0, 255)));
        result.insert("pencil", ("magenta", (255, 0, 255)));
        result.insert("clock", ("lightgray", (211, 211, 211)));
        result.insert("?", ("red", (255, 0, 0)));

        result
    };
}
