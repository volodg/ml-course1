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
                let position = [event.offset_x(), event.offset_y()];
                app_state.borrow_mut().update_points(&position);
                app_state.borrow().redraw().expect("")
            })
    }
}
