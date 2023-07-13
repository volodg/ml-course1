use crate::app_state::{ReadyState, SavedState};
use crate::html::HtmlDom;
use commons::utils::OkExt;
use drawing_commons::DrawingData;
use js_sys::encode_uri_component;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::html::Visibility;
use web_sys::HtmlElement;

pub trait Save {
    type View;

    fn save(&self) -> Result<SavedState<Self::View>, JsValue>;
}

fn convert_to_save_format(input: &ReadyState<HtmlDom>) -> DrawingData {
    let session = input.session.clone();
    let student = input.student.clone();
    let drawings: HashMap<_, _> = input
        .drawings
        .iter()
        .map(|paths| {
            let label = paths.get_label().to_owned();
            let paths = paths
                .get_paths()
                .iter()
                .map(|path| path.iter().map(|point| [point.x, point.y]).collect())
                .collect();
            (label, paths)
        })
        .collect();
    DrawingData::create(session, student, drawings)
}

impl Save for ReadyState<HtmlDom> {
    type View = HtmlDom;

    fn save(&self) -> Result<SavedState<Self::View>, JsValue> {
        let document = &self.get_view().document;
        let element = document.create_element("a")?.dyn_into::<HtmlElement>()?;

        let drawings = convert_to_save_format(self);

        let json = serde_json::to_string(&drawings)
            .map_err(|err| JsValue::from_str(std::format!("json error: {}", err).as_str()))?;
        let json = encode_uri_component(json.as_str());

        let attribute = std::format!("data:text/plain;charset=utf-8,{}", json);
        _ = element.set_attribute("href", attribute.as_str());

        let file_name = std::format!("{}.json", self.session);
        _ = element.set_attribute("download", file_name.as_str());

        element.set_display(false);

        let body = document.body().unwrap();
        _ = body.append_child(&element);
        element.click();
        _ = body.remove_child(&element);

        SavedState::create(self).ok()
    }
}
