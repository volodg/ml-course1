use commons::math::lerp::inv_lerp;
use commons::math::min_max;
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
        let cell_size = self.size as f64 / (cells_row_count as f64 + 1.0);

        let table = self
            .document
            .create_element("table")
            .expect("")
            .dyn_into::<HtmlElement>()?;
        table.style().set_property("border-collapse", "collapse")?;
        table.style().set_property("text-align", "center")?;
        table
            .style()
            .set_property("margin-left", std::format!("{cell_size}px").as_str())?;
        table
            .style()
            .set_property("margin-top", std::format!("{cell_size}px").as_str())?;

        self.container.append_child(&table)?;

        {
            let top_text = self
                .document
                .create_element("div")?
                .dyn_into::<HtmlElement>()?;
            top_text.set_inner_html("Predicted Class");
            top_text.style().set_property("position", "absolute")?;
            top_text.style().set_property("font-size", "x-large")?;
            top_text.style().set_property("top", "0")?;
            top_text.style().set_property("left", "50%")?;
            top_text
                .style()
                .set_property("transform", "translate(-50%)")?;
            top_text
                .style()
                .set_property("height", std::format!("{cell_size}px").as_str())?;
            top_text.style().set_property("display", "flex")?;
            top_text.style().set_property("align-items", "center")?;
            top_text.style().set_property(
                "margin-left",
                std::format!("{}px", cell_size / 2.0).as_str(),
            )?;

            self.container.append_child(&top_text)?;

            let left_text = self
                .document
                .create_element("div")?
                .dyn_into::<HtmlElement>()?;
            left_text.set_inner_html("True Class");
            left_text.style().set_property("position", "absolute")?;
            left_text.style().set_property("font-size", "x-large")?;
            left_text.style().set_property("top", "50%")?;
            left_text.style().set_property("left", "0")?;
            left_text
                .style()
                .set_property("transform", "translate(-50%) rotate(-90deg)")?;
            left_text
                .style()
                .set_property("height", std::format!("{cell_size}px").as_str())?;
            left_text.style().set_property("display", "flex")?;
            left_text.style().set_property("align-items", "center")?;
            left_text.style().set_property(
                "margin-left",
                std::format!("{}px", cell_size / 2.0).as_str(),
            )?;

            self.container.append_child(&left_text)?;
        }

        let matrix = self.prepare_matrix(cells_row_count);

        self.fill_table(table, cells_row_count, cell_size, matrix)?;

        Ok(())
    }

    fn prepare_matrix(&self, cells_row_count: usize) -> Vec<Vec<i64>> {
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

        for i in 1..cells_row_count {
            for j in 1..cells_row_count {
                result[0][j] += result[i][j];
                result[i][0] += result[i][j];
            }
        }

        for i in 1..cells_row_count {
            result[0][i] -= result[i][0];
        }

        result
    }

    fn fill_table(
        &self,
        table: HtmlElement,
        cells_row_count: usize,
        cell_size: f64,
        matrix: Vec<Vec<i64>>,
    ) -> Result<(), JsValue> {
        let zero_min_max: Option<i64> = None;
        let (min, max) = matrix[1..].iter().map(|x| &x[1..]).flatten().fold(
            (zero_min_max, zero_min_max),
            |(min, max), el| {
                let minmax = min_max((min, max), *el);
                (Some(minmax.0), Some(minmax.1))
            },
        );

        for i in 0..cells_row_count {
            let row = self.document.create_element("tr")?;
            table.append_child(&row)?;

            for j in 0..cells_row_count {
                let cell = self
                    .document
                    .create_element("td")?
                    .dyn_into::<HtmlElement>()?;
                cell.style()
                    .set_property("width", std::format!("{cell_size}px").as_str())?;
                cell.style()
                    .set_property("height", std::format!("{cell_size}px").as_str())?;
                cell.style().set_property("padding", "0")?;

                let text = if i == 0 && j == 0 {
                    "".to_string()
                } else if i == 0 && j > 0 {
                    let value = matrix[i][j];
                    if value > 0 {
                        std::format!("+{value}")
                    } else {
                        value.to_string()
                    }
                } else {
                    matrix[i][j].to_string()
                };

                cell.set_text_content(Some(text.as_str()));

                let img_src = |index: usize| -> String {
                    let image = self.styles[self.classes[index]].image.as_ref().expect("");
                    std::format!("url({})", image.src())
                };

                let image_src = if i == 0 && j > 0 {
                    Some(img_src(j - 1))
                } else if j == 0 && i > 0 {
                    Some(img_src(i - 1))
                } else {
                    None
                };

                if let Some(image_src) = image_src {
                    cell.style()
                        .set_property("background-image", image_src.as_str())?;
                    cell.style()
                        .set_property("background-repeat", "no-repeat")?;
                    cell.style()
                        .set_property("background-position", "50% 20%")?;
                    cell.style().set_property("vertical-align", "bottom")?;
                    cell.style().set_property("font-weight", "bold")?;
                }

                if i > 0 && j > 0 {
                    let proportion = inv_lerp(
                        min.expect("") as f64,
                        max.expect("") as f64,
                        matrix[i][j] as f64,
                    );

                    let color = if i == j {
                        std::format!("rgba(0, 0, 255, {})", proportion)
                    } else {
                        std::format!("rgba(255, 0, 0, {})", proportion)
                    };

                    cell.style()
                        .set_property("background-color", color.as_str())?;
                }

                row.append_child(&cell)?;
            }
        }

        Ok(())
    }
}
