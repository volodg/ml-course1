use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

pub trait DocumentExt {
    fn query_selector_each<F: FnMut(&Element) -> Result<(), JsValue>>(
        &self,
        selector: &str,
        visitor: F,
    ) -> Result<(), JsValue>;

    fn remove_all_classes(&self, class: &str) -> Result<(), JsValue>;
}

impl DocumentExt for Document {
    fn query_selector_each<F: FnMut(&Element) -> Result<(), JsValue>>(
        &self,
        selector: &str,
        mut visitor: F,
    ) -> Result<(), JsValue> {
        let selected = self.query_selector_all(selector)?;

        for i in 0..selected.length() {
            let element = selected.item(i).expect("").dyn_into::<Element>()?;
            visitor(&element)?;
        }

        Ok(())
    }

    fn remove_all_classes(&self, class: &str) -> Result<(), JsValue> {
        self.query_selector_each(std::format!(".{}", class).as_str(), |element| {
            element.class_list().remove_1(class)
        })
    }
}
