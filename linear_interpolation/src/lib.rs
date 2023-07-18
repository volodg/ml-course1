mod html;
mod app_state;

use crate::html::HtmlDom;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_commons::log;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let _html = HtmlDom::create()?;
    log("It works!!!");

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
