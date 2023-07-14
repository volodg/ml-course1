use crate::html::HtmlDom;
use drawing_commons::models::{FeaturesData, Sample};
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use plotly::common::{Marker, Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::html::InnerHtmlSetter;
use web_sys::HtmlImageElement;

pub trait Draw {
    fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue>;
    fn plot_statistic(&self, feature_data: &FeaturesData) -> Result<(), JsValue>;
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

    fn plot_statistic(&self, feature_data: &FeaturesData) -> Result<(), JsValue> {
        let html = plot_statistic_to_html(feature_data);

        let container = self.document.get_element_by_id("chartContainer").unwrap();
        container.set_inner_html_with_script(&html)
    }
}

fn plot_statistic_to_html(feature_data: &FeaturesData) -> String {
    let x_points = feature_data.features.iter().map(|x| {
        x.point[0]
    }).collect::<Vec<_>>();
    let y_points = feature_data.features.iter().map(|x| {
        x.point[1]
    }).collect::<Vec<_>>();

    let max_x = *(x_points.iter().max().unwrap_or(&0)) as f64 + 10.;
    let max_y = *(y_points.iter().max().unwrap_or(&0)) as f64 + 10.;

    let trace = Scatter::new(x_points, y_points)
        .mode(Mode::Markers)
        .name("Team A")
        .marker(Marker::new().size(12));

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let x_axis_name = feature_data.feature_names[0].as_str();
    let y_axis_name = feature_data.feature_names[1].as_str();

    let layout = Layout::new()
        .title(Title::new("Data Labels Hover"))
        .x_axis(Axis::new().title(Title::new(x_axis_name)).range(vec![0., max_x]))
        .y_axis(Axis::new().title(Title::new(y_axis_name)).range(vec![0., max_y]));
    plot.set_layout(layout);
    plot.to_inline_html(Some("chart"))
}
