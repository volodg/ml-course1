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
        // let app_state = app_state.clone();
        let document = self.document.clone();
        document.add_listener("mousemove", move |event: MouseEvent| {
            let b = {
                let app_state = app_state.borrow();
                let mut b = app_state.point_b;
                b[0] = event.offset_x() - app_state.html.offset[0];
                b[1] = event.offset_y() - app_state.html.offset[1];
                b
            };
            app_state.borrow_mut().point_b = b;
            let app_state = app_state.borrow();
            app_state.redraw(&app_state).expect("")
        })
    }
}
