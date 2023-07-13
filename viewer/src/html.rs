use commons::utils::OkExt;
use drawing_commons::{Sample, IMG_DIR};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, Element, HtmlImageElement};

#[derive(Clone)]
pub struct HtmlDom {
    document: Document,
    container: Element,
}

impl HtmlDom {
    #[allow(dead_code)]
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let container = document.get_element_by_id("container").unwrap();

        Self {
            document,
            container,
        }
        .ok()
    }

    #[allow(dead_code)]
    pub fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue> {
        let row = self.document.create_element("div")?;
        let _ = row.class_list().add_1("row")?;
        let _ = self.container.append_child(&row);

        let row_label = self.document.create_element("div")?;
        row_label.set_inner_html(student_name);
        let _ = row_label.class_list().add_1("rowLabel")?;
        let _ = row.append_child(&row_label);

        for sample in samples {
            let img = self
                .document
                .create_element("img")?
                .dyn_into::<HtmlImageElement>()?;

            let path = std::format!("{}/{}.png", IMG_DIR, sample.id);
            img.set_src(path.as_str());
            let _ = img.class_list().add_1("thumb")?;
            let _ = row.append_child(&img);
        }

        Ok(())
    }
}
