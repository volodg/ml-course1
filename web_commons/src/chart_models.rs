use crate::graphics::ContextExt;
use commons::geometry::Point2D;
use commons::math::{min_max, Bounds};
use commons::utils::OkExt;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub struct DataTransformation {
    pub offset: Point2D,
    pub scale: f64,
}

#[derive(Default)]
pub struct DragInto {
    pub start: Point2D,
    pub end: Point2D,
    pub offset: Point2D,
    pub dragging: bool,
}

#[derive(Clone, PartialEq)]
pub struct Sample {
    pub id: usize,
    pub group_id: u64,
    pub group_name: String,
    pub truth: Option<String>,
    pub label: String,
    pub point: Point2D,
}

impl Sample {
    pub fn create(
        id: usize,
        group_id: u64,
        group_name: String,
        label: String,
        point: Point2D,
    ) -> Self {
        Self {
            id,
            group_id,
            group_name,
            truth: None,
            label,
            point,
        }
    }

    pub fn correct(&self) -> bool {
        self.truth
            .as_ref()
            .map(|x| x.eq(&self.label))
            .unwrap_or(true)
    }
}

#[derive(Clone)]
pub struct SampleStyle {
    pub color: String,
    pub text: String,
    pub image: Option<HtmlImageElement>,
}

#[derive(Clone, PartialEq)]
pub enum SampleStyleType {
    Image,
    Text,
    Dot,
}

#[derive(Clone)]
pub struct Options {
    pub size: usize,
    pub axis_labels: [String; 2],
    pub styles: HashMap<String, SampleStyle>,
    pub icon: SampleStyleType,
    pub transparency: Option<f64>,
    pub background: Option<HtmlImageElement>,
}

impl Options {
    pub fn create(
        size: usize,
        axis_labels: [String; 2],
        styles: HashMap<String, SampleStyle>,
        icon: SampleStyleType,
        transparency: Option<f64>,
        background: Option<HtmlImageElement>,
    ) -> Result<Self, JsValue> {
        let mut result = Self {
            size,
            axis_labels,
            styles,
            icon,
            transparency,
            background,
        };

        if result.icon == SampleStyleType::Image {
            CanvasRenderingContext2d::generate_images(&mut result.styles)?;
        }

        result.ok()
    }
}

pub fn get_data_bounds(samples: &[Sample]) -> Bounds {
    let zero_min_max: Option<f64> = None;
    let (min_x, max_x, min_y, max_y) = samples.iter().fold(
        (zero_min_max, zero_min_max, zero_min_max, zero_min_max),
        |(min_x, max_x, min_y, max_y), el| {
            let x_minmax = min_max((min_x, max_x), el.point.x);
            let y_minmax = min_max((min_y, max_y), el.point.y);
            (
                Some(x_minmax.0),
                Some(x_minmax.1),
                Some(y_minmax.0),
                Some(y_minmax.1),
            )
        },
    );

    let delta_x = max_x.expect("") - min_x.expect("");
    let delta_y = max_y.expect("") - min_y.expect("");
    let _max_delta = delta_x.max(delta_y);

    Bounds {
        left: min_x.expect(""),
        right: max_x.expect(""), // min_x.expect("") + max_delta,
        top: max_y.expect(""),   // min_y.expect("") + max_delta,
        bottom: min_y.expect(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::chart_models::{get_data_bounds, Bounds, Sample};
    use commons::geometry::Point2D;

    #[test]
    fn test_data_bounds() {
        let samples = [
            Sample {
                id: 0,
                group_id: 0,
                group_name: 0.to_string(),
                truth: None,
                label: "label1".to_owned(),
                point: Point2D { x: 1.0, y: 10.0 },
            },
            Sample {
                id: 1,
                group_id: 0,
                group_name: 0.to_string(),
                truth: None,
                label: "label2".to_owned(),
                point: Point2D { x: 11.0, y: 2.0 },
            },
        ];

        let result = get_data_bounds(&samples);
        assert_eq!(
            result,
            Bounds {
                left: 1.0,
                right: 11.0,
                top: 10.0,
                bottom: 2.0,
            }
        );
    }
}
