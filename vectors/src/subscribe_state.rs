use crate::app_state::AppState;
use crate::draw::Draw;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::html::AddListener;
use web_sys::MouseEvent;

pub trait StateSubscriber {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

impl StateSubscriber for HtmlDom {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.document
            .add_listener("mousemove", move |event: MouseEvent| {
                app_state.borrow_mut().point = [event.offset_x() as f64, event.offset_y() as f64];
                app_state.borrow().draw().expect("");
            })
    }
}
