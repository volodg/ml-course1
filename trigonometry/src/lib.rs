mod app_state;
mod app_state_draw;
mod canvas;
mod canvas_chart;
mod draw;
mod html;
mod html_draw;
mod subscribe_state;

use crate::app_state::AppState;
use crate::html::HtmlDom;
use crate::subscribe_state::StateSubscriber;
use draw::Draw;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create()?;

    let app_state = AppState::create(html);
    app_state.draw()?;

    let app_state = Rc::new(RefCell::new(app_state));
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
