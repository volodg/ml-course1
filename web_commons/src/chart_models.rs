use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Bounds {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

#[derive(Clone)]
pub struct Sample {
    pub id: i32,
    pub label: String,
    pub point: Point,
}

impl Sample {
    pub fn create(id: i32, label: String, point: Point) -> Self {
        Self { id, label, point }
    }
}

pub struct Options {
    pub size: u32,
    pub axis_labels: [String; 2],
    pub styles: HashMap<String, String>,
}

pub fn get_data_bounds(samples: &[Sample]) -> Bounds {
    let zero_min_max: Option<f64> = None;
    fn min_max(
        (acc_min, acc_max): (Option<f64>, Option<f64>),
        el: f64,
    ) -> (Option<f64>, Option<f64>) {
        (
            Some(acc_min.map(|x| x.min(el)).unwrap_or(el)),
            Some(acc_max.map(|x| x.max(el)).unwrap_or(el)),
        )
    }
    let (min_x, max_x, min_y, max_y) = samples
        .iter()
        .fold((zero_min_max, zero_min_max, zero_min_max, zero_min_max), |(min_x, max_x, min_y, max_y), el| {
            let x_minmax = min_max((min_x, max_x), el.point.x);
            let y_minmax = min_max((min_y, max_y), el.point.y);
            (x_minmax.0, x_minmax.1, y_minmax.0, y_minmax.1)
        });
    Bounds {
        left: min_x.expect(""),
        right: max_x.expect(""),
        top: max_y.expect(""),
        bottom: min_y.expect(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::chart_models::{Bounds, get_data_bounds, Point, Sample};

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
