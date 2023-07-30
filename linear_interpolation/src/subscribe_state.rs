use crate::app_state::AppState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::subscribers::AddListener;
use web_sys::MouseEvent;

pub trait StateSubscriber {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

impl StateSubscriber for HtmlDom {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        self.document
            .on_click(move |_event: MouseEvent| app_state.borrow_mut().init_audio())
    }
}
