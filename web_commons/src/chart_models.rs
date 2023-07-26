use commons::math::{min_max, Bounds, Point};
use std::collections::HashMap;
use web_sys::HtmlImageElement;

pub struct DataTransformation {
    pub offset: Point,
    pub scale: f64,
}

#[derive(Default)]
pub struct DragInto {
    pub start: Point,
    pub end: Point,
    pub offset: Point,
    pub dragging: bool,
}

#[derive(Clone, PartialEq)]
pub struct Sample {
    pub id: usize,
    pub label: String,
    pub point: Point,
}

impl Sample {
    pub fn create(id: usize, label: String, point: Point) -> Self {
        Self { id, label, point }
    }
}

pub struct SampleStyle {
    pub color: String,
    pub text: String,
    pub image: Option<HtmlImageElement>,
}

#[derive(PartialEq)]
pub enum SampleStyleType {
    Image,
    Text,
    Dot,
}

pub struct Options {
    pub size: u32,
    pub axis_labels: [String; 2],
    pub styles: HashMap<String, SampleStyle>,
    pub icon: SampleStyleType,
    pub transparency: Option<f64>,
}

pub fn get_data_bounds(samples: &[Sample]) -> Bounds {
    let zero_min_max: Option<f64> = None;
    let (min_x, max_x, min_y, max_y) = samples.iter().fold(
        (zero_min_max, zero_min_max, zero_min_max, zero_min_max),
        |(min_x, max_x, min_y, max_y), el| {
            let x_minmax = min_max((min_x, max_x), el.point.x);
            let y_minmax = min_max((min_y, max_y), el.point.y);
            (Some(x_minmax.0), Some(x_minmax.1), Some(y_minmax.0), Some(y_minmax.1))
        },
    );
    Bounds {
        left: min_x.expect(""),
        right: max_x.expect(""),
        top: max_y.expect(""),
        bottom: min_y.expect(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::chart_models::{get_data_bounds, Bounds, Point, Sample};

    #[test]
    fn test_data_bounds() {
        let samples = [
            Sample {
                id: 0,
                label: "label1".to_owned(),
                point: Point { x: 1.0, y: 10.0 },
            },
            Sample {
                id: 1,
                label: "label2".to_owned(),
                point: Point { x: 11.0, y: 2.0 },
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
