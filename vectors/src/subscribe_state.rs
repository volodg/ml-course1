use crate::app_state::AppState;
use crate::draw::Draw;
use crate::html::HtmlDom;
use crate::vector::VectorXY;
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
            .add_listener("mousemove", move |event: MouseEvent| {
                let offset = app_state.borrow().html.canvas.offset;
                app_state.borrow_mut().point = VectorXY::new(
                    event.offset_x() as f64 - offset[0],
                    event.offset_y() as f64 - offset[0],
                );
                app_state.borrow().draw()
            })
    }
}
