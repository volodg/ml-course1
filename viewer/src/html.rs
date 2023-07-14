use commons::utils::OkExt;
use js_sys::eval;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, Element, HtmlScriptElement};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub container: Element,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let container = document.get_element_by_id("container").unwrap();

        Self {
            document,
            container,
        }
        .ok()
    }

    // TODO move to comments
    pub fn set_inner_html_with_script(
        &self,
        container: Element,
        html: &str,
    ) -> Result<(), JsValue> {
        container.set_inner_html(&html);

        let collection = container.get_elements_by_tag_name("script");
        for i in 0..collection.length() {
            let script = collection
                .item(i)
                .expect("")
                .dyn_into::<HtmlScriptElement>()?;
            let text: &str = &script.text().unwrap();
            eval(text)?;
        }

        Ok(())
    }
}
