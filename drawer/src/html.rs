use crate::app_state::WithStudent;
use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    window, CanvasRenderingContext2d, Document, HtmlButtonElement, HtmlCanvasElement,
    HtmlInputElement, HtmlSpanElement,
};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
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

        Self {
            document,
            student_input,
            advance_btn,
            undo_btn,
            instructions_spn,
            context,
            canvas,
        }
        .ok()
    }
}

impl WithStudent for HtmlDom {
    fn get_student(&self) -> String {
        self.student_input.value().trim().to_owned()
    }
}
