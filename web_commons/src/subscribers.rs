use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{EventTarget, HtmlElement, MouseEvent};

pub trait AddListener {
    fn add_listener<Event: wasm_bindgen::convert::FromWasmAbi + 'static, F>(
        &self,
        name: &str,
        listener: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Event) -> Result<(), JsValue> + 'static;

    fn on_click<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(MouseEvent) -> Result<(), JsValue> + 'static;
}

impl AddListener for EventTarget {
    fn add_listener<Event: wasm_bindgen::convert::FromWasmAbi + 'static, F>(
        &self,
        name: &str,
        mut listener: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Event) -> Result<(), JsValue> + 'static,
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| listener(event).expect(""));
        self.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }

    fn on_click<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(MouseEvent) -> Result<(), JsValue> + 'static,
    {
        self.add_listener("click", listener)
    }
}

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
