use commons::utils::OkExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart_models::{Options, Sample, SampleStyle};
use web_sys::{Document, Element, HtmlElement};

// TODO move it to web commons
pub struct Confusion {
    document: Document,
    container: Element,
    size: usize,
    #[allow(dead_code)]
    styles: HashMap<String, SampleStyle>,
    samples: Vec<Sample>,
    classes: Vec<&'static str>,
}

impl Confusion {
    pub fn create(
        document: Document,
        container: Element,
        options: Options,
    ) -> Result<Rc<RefCell<Self>>, JsValue> {
        let result = Self {
            document,
            container,
            size: options.size,
            styles: options.styles,
            samples: vec![],
            classes: vec![],
        };

        Rc::new(RefCell::new(result)).ok()
    }

    pub fn set_samples(&mut self, samples: &[Sample], classes: &[&'static str]) {
        self.samples = samples.into();
        self.classes = classes.into()
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let cells_row_count = self.classes.len() + 1;
        let cell_size = self.size / (cells_row_count + 1);

        let table = self
            .document
            .create_element("table")
            .expect("")
            .dyn_into::<HtmlElement>()?;
        table.style().set_property("borderCollapse", "collapse")?;
        table.style().set_property("textAlign", "center")?;
        table
            .style()
            .set_property("marginLeft", std::format!("{cell_size}px").as_str())?;
        table
            .style()
            .set_property("marginTop", std::format!("{cell_size}px").as_str())?;

        self.container.append_child(&table)?;

        let matrix = self.prepare_matrix(cells_row_count);

        self.fill_table(matrix);

        Ok(())
    }

    fn prepare_matrix(&self, cells_row_count: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::with_capacity(cells_row_count);

        for _ in 0..cells_row_count {
            let mut row = Vec::with_capacity(cells_row_count);

            for _ in 0..cells_row_count {
                row.push(0)
            }

            result.push(row)
        }

        for sample in &self.samples {}

        result
    }

    fn fill_table(&self, _matrix: Vec<Vec<usize>>) {}
}
