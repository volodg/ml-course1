use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
