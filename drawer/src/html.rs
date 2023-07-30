use crate::app_state::WithStudent;
use commons::utils::OkExt;
use drawing_commons::sketch_pad::SketchPad;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, HtmlButtonElement, HtmlInputElement, HtmlSpanElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub student_input: HtmlInputElement,
    pub advance_btn: HtmlButtonElement,
    pub instructions_spn: HtmlSpanElement,
    pub sketch_pad: Rc<RefCell<SketchPad>>,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
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

        let sketch_pad = SketchPad::create("inputContainer")?;

        Self {
            document,
            student_input,
            advance_btn,
            instructions_spn,
            sketch_pad,
        }
        .ok()
    }
}

impl WithStudent for HtmlDom {
    fn get_student(&self) -> String {
        self.student_input.value().trim().to_owned()
    }
}
