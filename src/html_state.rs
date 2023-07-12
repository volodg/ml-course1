use crate::app_state::{ReadyState, SavedState};
use crate::html::{HtmlDom, Visibility};
use js_sys::encode_uri_component;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;

pub trait Save {
    type View;

    fn save(&self) -> Result<SavedState<Self::View>, JsValue>;
}

impl Save for ReadyState<HtmlDom> {
    type View = HtmlDom;

    fn save(&self) -> Result<SavedState<Self::View>, JsValue> {
        let element = self.get_view().document.create_element("a")?.dyn_into::<HtmlElement>()?;

        let drawings: Vec<_> = self.drawings.iter().map(|x| {
            x.get_paths()
        }).collect();

        let json = serde_json::to_string(&drawings).unwrap();
        let json = encode_uri_component(json.as_str());

        let attribute = std::format!("data:text/plain;charset=utf-8,{}", json);
        _ = element.set_attribute("href", attribute.as_str());

        let file_name = "todo_change_me.json";
        _ = element.set_attribute("download", file_name);

        element.set_display(false);

        _ = self.get_view().document.body().unwrap().append_child(&element);
        element.click();
        _ = self.get_view().document.body().unwrap().remove_child(&element);

        Ok(SavedState::create(self))
    }
}
