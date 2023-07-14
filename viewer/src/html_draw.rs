use plotters::prelude::{BLUE, ChartBuilder, Circle, Color, IntoDrawingArea, IntoFont, WHITE};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use drawing_commons::models::{FeaturesData, Sample};
use wasm_bindgen::JsCast;
use crate::html::HtmlDom;

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Draw {
    fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue>;
    fn draw_chart(&self, features: &FeaturesData) -> DrawResult<()>;
}

impl Draw for HtmlDom {
    fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue> {
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

            let sample_container = self.document.create_element("div")?;
            sample_container.set_id(std::format!("sample_{}", sample.id).as_str());
            let _ = sample_container.class_list().add_1("sampleContainer")?;

            let sample_label = self.document.create_element("div")?;
            sample_label.set_inner_html(sample.label.as_str());
            let _ = sample_container.append_child(&sample_label);

            let path = std::format!("{}/{}.png", IMG_DIR, sample.id);
            img.set_src(path.as_str());
            let _ = img.class_list().add_1("thumb")?;
            if FLAGGED_USERS.contains(&sample.student_id) {
                let _ = img.class_list().add_1("blur")?;
            }
            let _ = sample_container.append_child(&img);

            let _ = row.append_child(&sample_container);
        }

        Ok(())
    }

    // TODO Try:
    // 1. https://igiagkiozis.github.io/plotly
    // https://github.com/igiagkiozis/plotly#usage-within-a-wasm-environment
    // 2. https://crates.io/crates/poloto
    // 3. https://crates.io/crates/graplot
    fn draw_chart(&self, features: &FeaturesData) -> DrawResult<()> {
        let root = CanvasBackend::with_canvas_object(self.canvas.clone())
            .unwrap()
            .into_drawing_area();

        root.fill(&WHITE)?;

        let random_points: Vec<(f64, f64)> = features.features.iter().map(|x| {
            (x.point[0] as f64, x.point[1] as f64)
        }).collect();

        let areas = root.split_by_breakpoints([1000], [50]);

        let caption = std::format!("({},{})", features.feature_names[0], features.feature_names[1]);

        let mut scatter_ctx = ChartBuilder::on(&areas[2])
            .caption(caption, ("sans-serif", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0f64..300f64, 0f64..20_000f64)?;
        scatter_ctx
            .configure_mesh()
            .draw()?;
        scatter_ctx.draw_series(
            random_points
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
        )?;

        Ok(())
    }
}
