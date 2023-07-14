use crate::html::HtmlDom;
use drawing_commons::models::{FeaturesData, Sample};
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use plotly::common::{Marker, Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};
use std::collections::HashMap;
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

#[derive(Default)]
struct TracesData {
    traces: HashMap<String, (Vec<usize>, Vec<usize>)>,
    max: [usize; 2],
}

impl TracesData {
    fn into(self) -> Vec<Box<Scatter<usize, usize>>> {
        self.traces
            .into_iter()
            .map(|(key, values)| {
                Scatter::new(values.0, values.1)
                    .mode(Mode::Markers)
                    .name(key)
                    .marker(Marker::new().size(12))
            })
            .collect()
    }
}

fn feature_data_into_traces(feature_data: &FeaturesData) -> TracesData {
    feature_data
        .features
        .iter()
        .fold(TracesData::default(), |acc, el| {
            let TracesData {
                mut traces,
                mut max,
            } = acc;

            let (x_values, y_values) = traces.entry(el.label.clone()).or_default();
            x_values.push(el.point[0]);
            y_values.push(el.point[1]);

            max[0] = max[0].max(el.point[0]);
            max[1] = max[1].max(el.point[1]);

            TracesData { traces, max }
        })
}

fn plot_statistic_to_html(feature_data: &FeaturesData) -> String {
    let traces_data = feature_data_into_traces(feature_data);

    let max_x = traces_data.max[0] as f64 + 10.;
    let max_y = traces_data.max[1] as f64 + 10.;

    let traces = traces_data.into();

    let mut plot = Plot::new();
    for trace in traces {
        plot.add_trace(trace);
    }

    let x_axis_name = feature_data.feature_names[0].as_str();
    let y_axis_name = feature_data.feature_names[1].as_str();

    let layout = Layout::new()
        .title(Title::new("Features statistic"))
        .x_axis(
            Axis::new()
                .title(Title::new(x_axis_name))
                .range(vec![0., max_x]),
        )
        .y_axis(
            Axis::new()
                .title(Title::new(y_axis_name))
                .range(vec![0., max_y]),
        );
    plot.set_layout(layout);
    plot.to_inline_html(Some("chart"))
}
