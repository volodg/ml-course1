use crate::app_state::AppState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use web_commons::subscribers::AddListener;
use wasm_bindgen::JsValue;
use web_sys::HtmlButtonElement;
use crate::storage::{discard_best_brain, save_best_brain};

pub trait StateSubscriber {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

impl StateSubscriber for HtmlDom {
    fn subscribe(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let document = &app_state.borrow().html.document;

        let save_btn = document
            .get_element_by_id("saveButton")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;

        let app_state_copy = app_state.clone();
        save_btn.on_click(move |_| {
            if let Some(brane) = &app_state_copy.borrow().best_car {
                save_best_brain(brane)?
            }
            Ok(())
        })?;

        let discard_btn = document
            .get_element_by_id("discardButton")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;

        discard_btn.on_click(move |_| {
            discard_best_brain()
        })?;

        Ok(())
    }
}
