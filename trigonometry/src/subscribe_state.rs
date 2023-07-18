use crate::app_state::AppState;
use crate::draw::Draw;
use crate::html::HtmlDom;
use js_sys::Math::sign;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::html::AddListener;
use web_sys::WheelEvent;

pub trait StateSubscriber {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

impl StateSubscriber for HtmlDom {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.document
            .add_listener("onwheel", move |event: WheelEvent| {
                {
                    let mut borrow = app_state.borrow_mut();
                    borrow.theta -= sign(event.delta_y());
                    borrow.update_points();
                }
                app_state.borrow().draw().expect("")
            })
    }
}
