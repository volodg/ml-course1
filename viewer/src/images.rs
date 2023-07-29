use commons::utils::OkExt;
use drawing_commons::data::IMAGE_SRC;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, HtmlImageElement};

pub fn create_background_image() -> Result<HtmlImageElement, JsValue> {
    let document = window().expect("").document().expect("");

    let image = document
        .create_element("img")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();
    image.set_src(&IMAGE_SRC);

    image.ok()
}
