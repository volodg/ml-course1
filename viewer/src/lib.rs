use drawing_commons::Sample;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use web_commons::log;

lazy_static! {
    static ref SAMPLES_DATA: Vec<Sample> =
        serde_json::from_str::<Vec<Sample>>(std::include_str!("../../data/dataset/samples.json"))
            .expect("");
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let size = SAMPLES_DATA.len();
    log(std::format!("Hi from Rust: {:?}", size).as_str());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::SAMPLES_DATA;

    #[test]
    fn test_samples() {
        let size = SAMPLES_DATA.len();
        assert_eq!(size, 5728);
    }
}
