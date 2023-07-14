mod html;
mod html_draw;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use crate::html::HtmlDom;
use crate::html_draw::Draw;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let hml = HtmlDom::create()?;
    hml.draw()?;

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
