use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, HtmlTableElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub data_table: HtmlTableElement,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        let data_table = document.get_element_by_id("dataTable").unwrap();
        let data_table = data_table.dyn_into::<HtmlTableElement>()?;

        Self {
            document,
            data_table,
        }
            .ok()
    }
}
