use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, HtmlElement,
    HtmlInputElement, HtmlSpanElement, MouseEvent,
};

pub struct HtmlDom {
    pub student_input: HtmlInputElement,
    pub advance_btn: HtmlButtonElement,
    pub undo_btn: HtmlButtonElement,
    pub instructions_spn: HtmlSpanElement,
    pub context: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let undo_btn = document
            .get_element_by_id("undo")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;
        let student_input = document
            .get_element_by_id("student")
            .unwrap()
            .dyn_into::<HtmlInputElement>()?;
        let advance_btn = document
            .get_element_by_id("advanceBtn")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;
        let instructions_spn = document
            .get_element_by_id("instructions")
            .unwrap()
            .dyn_into::<HtmlSpanElement>()?;

        Ok(Self {
            student_input,
            advance_btn,
            undo_btn,
            instructions_spn,
            context,
            canvas,
        })
    }
}

pub trait Visibility {
    fn set_visible(&self, visible: bool);
    fn set_display(&self, visible: bool);
}

impl Visibility for HtmlElement {
    fn set_visible(&self, visible: bool) {
        if visible {
            self.style().set_property("visibility", "visible").unwrap();
        } else {
            self.style().set_property("visibility", "hidden").unwrap();
        }
    }

    fn set_display(&self, display: bool) {
        if display {
            self.style().remove_property("display").unwrap();
        } else {
            self.style().set_property("display", "none").unwrap();
        }
    }
}

pub fn alert(msg: &str) {
    if let Some(window) = window() {
        let _ = window.alert_with_message(msg);
    }
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

impl AddListener for HtmlElement {
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
