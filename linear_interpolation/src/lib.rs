mod app_state;
mod app_state_draw;
mod draw;
mod html;
mod html_draw;
mod subscribe_state;

use crate::app_state::AppState;
use crate::draw::Draw;
use crate::html::HtmlDom;
use crate::subscribe_state::StateSubscriber;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create()?;

    let app_state = AppState::create(html);
    let app_state = Rc::new(RefCell::new(app_state));
    app_state.borrow().draw(app_state.clone())?;
    app_state.borrow().html.subscribe(app_state.clone())?;

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
