use crate::graphics::ContextExt;
use commons::geometry::{Point2D, PointN};
use commons::math::{min_max, Bounds};
use commons::utils::{OkExt, SomeExt};
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
    pub point: PointN,
}

impl Sample {
    pub fn create(
        id: usize,
        group_id: u64,
        group_name: String,
        label: String,
        point: PointN,
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

pub fn get_data_bounds(samples: &[Sample]) -> Option<Bounds> {
    let zero_min_max: Option<f64> = None;
    let (min_x, max_x, min_y, max_y) = samples.iter().fold(
        (zero_min_max, zero_min_max, zero_min_max, zero_min_max),
        |(min_x, max_x, min_y, max_y), el| {
            let x_minmax = min_max((min_x, max_x), el.point[0]);
            let y_minmax = min_max((min_y, max_y), el.point[1]);
            (
                Some(x_minmax.0),
                Some(x_minmax.1),
                Some(y_minmax.0),
                Some(y_minmax.1),
            )
        },
    );

    match (min_x, max_x, max_y, min_y) {
        (Some(min_x), Some(max_x), Some(max_y), Some(min_y)) => {
            Bounds::create(min_x, max_x, max_y, min_y).some()
        }
        (_, _, _, _) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::chart_models::{get_data_bounds, Bounds, Sample};
    use commons::utils::SomeExt;

    #[test]
    fn test_data_bounds() {
        let samples = [
            Sample {
                id: 0,
                group_id: 0,
                group_name: 0.to_string(),
                truth: None,
                label: "label1".to_owned(),
                point: vec![1.0, 10.0],
            },
            Sample {
                id: 1,
                group_id: 0,
                group_name: 0.to_string(),
                truth: None,
                label: "label2".to_owned(),
                point: vec![11.0, 2.0],
            },
        ];

        let result = get_data_bounds(&samples);
        assert_eq!(result, Bounds::create(1.0, 11.0, 10.0, 2.0,).some());

        let result = get_data_bounds(&[]);
        assert_eq!(result, None);
    }
}
