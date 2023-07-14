use plotters::prelude::{BLACK, ChartBuilder, Color, IntoDrawingArea, IntoFont, LineSeries, PathElement, RED, WHITE};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use drawing_commons::models::Sample;
use wasm_bindgen::JsCast;
use crate::html::HtmlDom;

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Draw {
    fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue>;
    fn draw_chart(&self) -> DrawResult<()>;
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

    fn draw_chart(&self) -> DrawResult<()> {
        let root = CanvasBackend::with_canvas_object(self.canvas.clone())
            .unwrap()
            .into_drawing_area();

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))?
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}
