use crate::html::HtmlDom;
use commons::math::Point;
use drawing_commons::models::{FeaturesData, Sample};
use drawing_commons::{FLAGGED_USERS, IMG_DIR};
use lazy_static::lazy_static;
use palette::{named::from_str, Srgb};
use plotly::color::{NamedColor, Rgba};
use plotly::common::{Marker, Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::html::InnerHtmlSetter;
use web_sys::{window, HtmlImageElement, ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition, Element};

lazy_static! {
    static ref COLOR_STYLES: HashMap<String, (Rgba, String)> = (|| {
        let mut result = HashMap::new();

        result.insert("car".to_owned(), (NamedColor::Gray, "🚗".to_owned()));
        result.insert("fish".to_owned(), (NamedColor::Red, "🐟".to_owned()));
        result.insert("house".to_owned(), (NamedColor::Yellow, "🏠".to_owned()));
        result.insert("tree".to_owned(), (NamedColor::Green, "🌳".to_owned()));
        result.insert("bicycle".to_owned(), (NamedColor::Cyan, "🚲".to_owned()));
        result.insert("guitar".to_owned(), (NamedColor::Blue, "🎸".to_owned()));
        result.insert("pencil".to_owned(), (NamedColor::Magenta, "✏️".to_owned()));
        result.insert("clock".to_owned(), (NamedColor::LightGray, "⏰".to_owned()));

        result
            .into_iter()
            .map(|(key, (color, symbol))| {
                let color = from_named_srgb_color(&color);
                let color = Rgba::new(color.red, color.green, color.blue, 0.7);
                (key, (color, symbol))
            })
            .collect()
    })();
}

fn from_named_srgb_color(color: &NamedColor) -> Srgb<u8> {
    let value: serde_json::Value = serde_json::to_value(&color).expect("");
    let color = value.as_str().expect("");
    from_str(&color).unwrap()
}

pub trait Draw {
    fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue>;
    fn plot_statistic(&self, feature_data: &FeaturesData) -> Result<(), JsValue>;
    fn plot_statistic2(&self, feature_data: &FeaturesData) -> Result<(), JsValue>;
}

impl Draw for HtmlDom {
    fn create_row(&self, student_name: &str, samples: &[&Sample]) -> Result<(), JsValue> {
        let row = self.document.create_element("div")?;
        row.class_list().add_1("row")?;
        _ = self.container.append_child(&row)?;

        let row_label = self.document.create_element("div")?;
        row_label.set_inner_html(student_name);
        row_label.class_list().add_1("rowLabel")?;
        _ = row.append_child(&row_label)?;

        for sample in samples {
            let img = self
                .document
                .create_element("img")?
                .dyn_into::<HtmlImageElement>()?;

            let sample_container = self.document.create_element("div")?;
            sample_container.set_id(std::format!("sample_{}", sample.id).as_str());
            _ = sample_container.class_list().add_1("sampleContainer")?;

            let sample_label = self.document.create_element("div")?;
            sample_label.set_inner_html(sample.label.as_str());
            _ = sample_container.append_child(&sample_label)?;

            let path = std::format!("{}/{}.png", IMG_DIR, sample.id);
            img.set_src(path.as_str());
            img.class_list().add_1("thumb")?;
            if FLAGGED_USERS.contains(&sample.student_id) {
                img.class_list().add_1("blur")?;
            }
            sample_container.append_child(&img)?;

            row.append_child(&sample_container)?;
        }

        Ok(())
    }

    fn plot_statistic(&self, feature_data: &FeaturesData) -> Result<(), JsValue> {
        let html = plot_statistic_to_html(feature_data);

        let container = self.document.get_element_by_id("chartContainer").unwrap();
        container.set_inner_html_with_script(&html)
    }

    fn plot_statistic2(&self, feature_data: &FeaturesData) -> Result<(), JsValue> {
        let mut chart = self.chart.borrow_mut();

        use web_commons::chart_models::Sample;

        let samples = feature_data
            .features
            .iter()
            .zip(1..)
            .map(|(feature, id)| Sample {
                id,
                label: feature.label.clone(),
                point: Point {
                    x: feature.point[0] as f64,
                    y: feature.point[1] as f64,
                },
            })
            .collect::<Vec<_>>();

        chart.set_samples(samples);

        let on_click_callback = Rc::new(RefCell::new(move |sample: Option<&Sample>| {
            handle_click(sample).expect("")
        }));

        chart.set_on_click(on_click_callback);

        chart.draw()
    }
}

fn handle_click(sample: Option<&web_commons::chart_models::Sample>) -> Result<(), JsValue> {
    let document = window().expect("").document().expect("");
    let selected = document.query_selector_all(".emphasize")?;

    let emphasize_class_name = "emphasize";

    let de_emphasize = || -> Result<(), JsValue> {
        for i in 0..selected.length() {
            let element = selected.item(i).expect("").dyn_into::<Element>()?;
            element.class_list().remove_1(emphasize_class_name)?;
        }
        Ok(())
    };

    de_emphasize()?;

    match sample {
        Some(sample) => {
            let element = document
                .get_element_by_id(std::format!("sample_{}", sample.id).as_str())
                .unwrap();
            element.class_list().add_1(emphasize_class_name)?;

            let mut options = ScrollIntoViewOptions::new();
            options.behavior(ScrollBehavior::Auto);
            options.block(ScrollLogicalPosition::Center);
            element.scroll_into_view_with_scroll_into_view_options(&options);
        }
        None => (),
    }

    Ok(())
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
                let (_, symbol) = COLOR_STYLES.get(&key).expect("");
                let marker = Marker::new().color(NamedColor::Transparent).size(12);
                Scatter::new(values.0, values.1)
                    .mode(Mode::MarkersText)
                    .name(key)
                    .text(symbol)
                    .marker(marker)
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

#[cfg(test)]
mod tests {
    use crate::html_draw::{from_named_srgb_color, COLOR_STYLES};
    use plotly::color::NamedColor;

    #[test]
    fn test_from_named_srgb_color() {
        let srgb = from_named_srgb_color(&NamedColor::Gray);
        assert_eq!(srgb.red, 128);
        assert_eq!(srgb.green, 128);
        assert_eq!(srgb.blue, 128);
    }

    #[test]
    fn test_color_styles() {
        let size = COLOR_STYLES.len();
        assert_eq!(size, 8);
    }
}
