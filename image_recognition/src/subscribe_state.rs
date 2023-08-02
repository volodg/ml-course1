use crate::app_state::AppState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

pub trait StateSubscriber {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

impl StateSubscriber for HtmlDom {
    fn subscribe(&self, _app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        // self.document
        //     .add_listener("mousemove", move |_event: MouseEvent| {
        //         // TODO
        //     })
        Ok(())
    }
}
