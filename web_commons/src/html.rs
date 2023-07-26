use js_sys::eval;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Element, EventTarget, HtmlElement, HtmlScriptElement, MouseEvent};

pub trait Visibility {
    fn set_visible(&self, visible: bool) -> Result<(), JsValue>;
    fn is_displayed(&self) -> bool;
    fn set_display(&self, visible: bool) -> Result<(), JsValue>;
}

impl Visibility for HtmlElement {
    fn set_visible(&self, visible: bool) -> Result<(), JsValue> {
        if visible {
            self.style().set_property("visibility", "visible")
        } else {
            self.style().set_property("visibility", "hidden")
        }
    }

    fn is_displayed(&self) -> bool {
        self.style()
            .get_property_value("display")
            .ok()
            .map(|x| x != "none")
            .unwrap_or(true)
    }

    fn set_display(&self, display: bool) -> Result<(), JsValue> {
        if display {
            self.style().remove_property("display")?;
            Ok(())
        } else {
            self.style().set_property("display", "none")
        }
    }
}

pub fn alert(msg: &str) -> Result<(), JsValue> {
    if let Some(window) = window() {
        return window.alert_with_message(msg);
    }
    Err(JsValue::from_str("no window"))
}

pub trait AddListener {
    fn add_listener<Event: wasm_bindgen::convert::FromWasmAbi + 'static, F>(
        &self,
        name: &str,
        listener: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Event) + 'static;

    fn on_click<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(MouseEvent) + 'static;
}

impl AddListener for EventTarget {
    fn add_listener<Event: wasm_bindgen::convert::FromWasmAbi + 'static, F>(
        &self,
        name: &str,
        mut listener: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Event) + 'static,
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| listener(event));
        self.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }

    fn on_click<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(MouseEvent) + 'static,
    {
        self.add_listener("click", listener)
    }
}

pub trait InnerHtmlSetter {
    fn set_inner_html_with_script(&self, html: &str) -> Result<(), JsValue>;
}

impl InnerHtmlSetter for Element {
    fn set_inner_html_with_script(&self, html: &str) -> Result<(), JsValue> {
        self.set_inner_html(&html);

        let collection = self.get_elements_by_tag_name("script");
        for i in 0..collection.length() {
            let script = collection
                .item(i)
                .expect("")
                .dyn_into::<HtmlScriptElement>()?;
            let text: &str = &script.text()?;
            eval(text)?;
        }

        Ok(())
    }
}
