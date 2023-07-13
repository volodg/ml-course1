use wasm_bindgen::prelude::*;
use web_commons::log;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    log("Hi from Rust");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
