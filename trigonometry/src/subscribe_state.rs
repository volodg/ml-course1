use crate::app_state::AppState;
use crate::draw::Draw;
use crate::html::HtmlDom;
use js_sys::Math::sign;
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
                let last_x_pos = app_state.borrow().last_x_pos;
                {
                    let mut borrow = app_state.borrow_mut();
                    borrow.theta -= sign(last_x_pos - event.offset_x() as f64) / 100.0;
                    borrow.last_x_pos = last_x_pos;
                    borrow.update_points();
                }
                app_state.borrow().draw().expect("")
            })
    }
}
