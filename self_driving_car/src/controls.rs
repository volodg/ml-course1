use crate::car::ControlType;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use wasm_bindgen::JsValue;
use web_commons::subscribers::AddListener;
use web_sys::{window, KeyboardEvent};

pub struct Controls {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
    pub reverse: bool,
    weak_self: Weak<RefCell<Controls>>,
}

impl Controls {
    pub fn create(control_type: ControlType) -> Result<Rc<RefCell<Self>>, JsValue> {
        let result = Self {
            forward: false,
            left: false,
            right: false,
            reverse: false,
            weak_self: Weak::new(),
        };

        let result = Rc::new(RefCell::new(result));
        result.borrow_mut().weak_self = Rc::downgrade(&result);

        match control_type {
            ControlType::Dummy => result.borrow_mut().forward = true,
            ControlType::Keys => result.borrow().add_keyboard_listeners()?,
            ControlType::AI => result.borrow().add_keyboard_listeners()?,
        }

        result.ok()
    }

    fn set_direction(&mut self, key: &str, value: bool) {
        match key {
            "ArrowLeft" => self.left = value,
            "ArrowRight" => self.right = value,
            "ArrowUp" => self.forward = value,
            "ArrowDown" => self.reverse = value,
            _ => (),
        }
    }

    fn add_keyboard_listeners(&self) -> Result<(), JsValue> {
        let document = window().expect("").document().expect("");

        // TODO use weak reference later
        let controls = self.weak_self.upgrade().expect("");
        document.add_listener("keydown", move |event: KeyboardEvent| {
            let mut controls = controls.borrow_mut();
            controls.set_direction(event.key().as_str(), true);
            event.prevent_default();

            Ok(())
        })?;

        let weak_self = self.weak_self.clone();
        document.add_listener("keyup", move |event: KeyboardEvent| {
            let binding = weak_self.upgrade().expect("");
            let mut controls = binding.borrow_mut();
            controls.set_direction(event.key().as_str(), false);
            event.prevent_default();

            Ok(())
        })
    }
}
