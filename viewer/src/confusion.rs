use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::chart_models::{Options, Sample};
use web_sys::Element;

// TODO move it to web commons
pub struct Confusion {
    #[allow(dead_code)]
    container: Element,
    #[allow(dead_code)]
    options: Options,
    #[allow(dead_code)]
    samples: Vec<Sample>,
    #[allow(dead_code)]
    classes: Vec<&'static str>,
}

impl Confusion {
    pub fn create(container: Element, options: Options) -> Result<Rc<RefCell<Self>>, JsValue> {
        let result = Self {
            container,
            options,
            samples: vec![],
            classes: vec![],
        };

        Rc::new(RefCell::new(result)).ok()
    }

    pub fn set_samples(&mut self, samples: &[Sample], classes: &[&'static str]) {
        self.samples = samples.into();
        self.classes = classes.into();
    }

    pub fn draw(&self) {

    }
}
