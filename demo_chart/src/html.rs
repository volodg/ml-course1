use crate::canvas::Canvas;
use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, HtmlTableElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub canvas: Canvas,
    pub data_table: HtmlTableElement,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = Canvas::create(&document, "chartContainer")?;

        let data_table = document.get_element_by_id("dataTable").unwrap();
        let data_table = data_table.dyn_into::<HtmlTableElement>()?;

        Self {
            document,
            canvas,
            data_table,
        }
        .ok()
    }
}
