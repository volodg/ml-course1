use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub trait HtmlElementExt {
    fn on_load<F>(&self, listener: F)
    where
        F: FnMut() + 'static;
}

impl HtmlElementExt for HtmlElement {
    fn on_load<F>(&self, mut listener: F)
    where
        F: FnMut() + 'static,
    {
        let closure = Closure::<dyn FnMut()>::new(move || listener());
        self.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget()
    }
}
