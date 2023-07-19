use crate::canvas::Canvas;
use commons::utils::OkExt;
use wasm_bindgen::JsValue;
use web_sys::{window, Document};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub canvas: Canvas,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = Canvas::create(&document, "myCanvas")?;

        Self { document, canvas }.ok()
    }
}
