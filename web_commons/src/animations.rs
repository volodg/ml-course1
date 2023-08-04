use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

pub fn animate_with_callback<F>(mut callback: F)
where
    F: FnMut() -> Result<(), JsValue> + 'static,
{
    let animation_f = Rc::new(RefCell::new(None));
    let animation_f_copy = animation_f.clone();

    *animation_f_copy.borrow_mut() = Some(Closure::new(move || {
        callback().expect("");
        request_animation_frame(animation_f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(animation_f_copy.borrow().as_ref().unwrap());
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
