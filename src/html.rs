use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, HtmlElement, HtmlInputElement, window};
use wasm_bindgen::JsCast;

pub struct Html {
    pub student_input: HtmlInputElement,
    pub advance_btn: HtmlButtonElement,
    pub undo_btn: HtmlButtonElement,
    pub context: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

impl Html {
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

        Ok(Self {
            student_input,
            advance_btn,
            undo_btn,
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
