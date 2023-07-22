use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, Element, HtmlTableElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub chart_container: Element,
    pub data_table: HtmlTableElement,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        let chart_container = document.get_element_by_id("chartContainer").unwrap();

        let data_table = document.get_element_by_id("dataTable").unwrap();
        let data_table = data_table.dyn_into::<HtmlTableElement>()?;

        Self {
            document,
            chart_container,
            data_table,
        }
        .ok()
    }
}
