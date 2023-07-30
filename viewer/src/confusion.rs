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

        {
            let top_text = self.document.create_element("div")?.dyn_into::<HtmlElement>()?;
            top_text.set_inner_html("Predicted Class");
            top_text.style().set_property("position", "absolute")?;
            top_text.style().set_property("fontSize", "x-large")?;
            top_text.style().set_property("top", "0")?;
            top_text.style().set_property("left", "50%")?;
            top_text.style().set_property("transform", "translate(-50%)")?;
            top_text.style().set_property("height", std::format!("{cell_size}px").as_str())?;
            top_text.style().set_property("display", "flex")?;
            top_text.style().set_property("alignItems", "center")?;
            top_text.style().set_property("marginLeft", std::format!("{}px", cell_size / 2).as_str())?;

            self.container.append_child(&top_text)?;
        }

        let matrix = self.prepare_matrix(cells_row_count);

        self.fill_table(table, cells_row_count, cell_size, matrix)?;

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

        for sample in &self.samples {
            let row = self
                .classes
                .iter()
                .position(|x| sample.truth.as_ref().expect("") == *x)
                .expect("");
            let column = self
                .classes
                .iter()
                .position(|x| sample.label == *x)
                .expect("");
            result[row + 1][column + 1] += 1;
        }

        result
    }

    fn fill_table(&self, table: HtmlElement, cells_row_count: usize, cell_size: usize, matrix: Vec<Vec<usize>>) -> Result<(), JsValue> {

        for i in 0..cells_row_count {
            let row = self.document.create_element("tr")?;
            table.append_child(&row)?;

            for j in 0..cells_row_count {
                let cell = self.document.create_element("td")?.dyn_into::<HtmlElement>()?;
                cell.style().set_property("width", std::format!("{cell_size}px").as_str())?;
                cell.style().set_property("height", std::format!("{cell_size}px").as_str())?;
                cell.style().set_property("padding", "0")?;

                cell.set_text_content(Some(matrix[i][j].to_string().as_str()));

                row.append_child(&cell)?;
            }
        }

        Ok(())
    }
}
