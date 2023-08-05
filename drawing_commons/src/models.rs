use commons::geometry::{graham_scan, minimum_bounding_box, polygon_roundness, Point2DView};
use commons::math::lerp::inv_lerp;
use commons::math::{min_max, min_max_n_points, Bounds};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct Sample {
    pub id: usize,
    pub label: String,
    pub student_name: String,
    pub student_id: u64,
}

type Drawings = HashMap<String, Vec<Vec<[f64; 2]>>>;

#[derive(Deserialize, Serialize)]
pub struct DrawingData {
    pub session: u64,
    pub student: String,
    pub drawings: Drawings,
}

impl DrawingData {
    pub fn create(session: u64, student: String, drawings: Drawings) -> Self {
        Self {
            session,
            student,
            drawings,
        }
    }

    pub fn get_student(&self) -> &String {
        &self.student
    }

    pub fn get_session(&self) -> u64 {
        self.session
    }

    pub fn get_drawings(&self) -> &Drawings {
        &self.drawings
    }
}

pub type DrawingPaths<T> = Vec<Vec<T>>;

pub trait Features {
    type ElType;

    fn path_count(&self) -> usize;

    fn point_count(&self) -> usize;

    fn get_width(&self, el_getter: impl Fn(&Self::ElType) -> f64) -> f64;

    fn get_pixels(&self, expand: bool) -> Vec<u8>;

    fn get_hull(&self) -> Vec<[f64; 2]>;

    fn get_feature(&self) -> Vec<f64>;
}

impl<T: Point2DView> Features for DrawingPaths<T> {
    type ElType = T;

    fn path_count(&self) -> usize {
        self.len()
    }

    fn point_count(&self) -> usize {
        self.iter().fold(0, |acc, drawing| acc + drawing.len())
    }

    fn get_width(&self, el_getter: impl Fn(&Self::ElType) -> f64) -> f64 {
        // TODO use min max
        let zero_min_max: Option<f64> = None;
        let (min_x, max_x) =
            self.iter()
                .flatten()
                .fold((zero_min_max, zero_min_max), |(min_x, max_x), el| {
                    let x_minmax = min_max((min_x, max_x), el_getter(el));
                    (Some(x_minmax.0), Some(x_minmax.1))
                });

        match (max_x, min_x) {
            (Some(max_x), Some(min_x)) => max_x.round() - min_x.round(),
            (_, _) => 0.0,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_pixels(&self, expand: bool) -> Vec<u8> {
        use crate::draw_images::DrawTargetExt;
        use raqote::DrawTarget;

        let size = 400;
        let mut dt = DrawTarget::new(size, size);

        if expand {
            let res = expand_path(self, size);
            dt.draw_path(&res, 3.0);
        } else {
            dt.draw_path(self, 3.0);
        }

        dt.get_data().iter().map(|x| (*x >> 24) as u8).collect()
    }

    #[cfg(target_arch = "wasm32")]
    fn get_pixels(&self, expand: bool) -> Vec<u8> {
        use crate::canvas_ext::CanvasRenderingContext2dExt;
        use wasm_bindgen::JsCast;
        use web_sys::window;
        use web_sys::CanvasRenderingContext2d;
        use web_sys::HtmlCanvasElement;

        let document = window().expect("").document().expect("");

        let canvas = document
            .create_element("canvas")
            .expect("")
            .dyn_into::<HtmlCanvasElement>()
            .expect("");
        let size = 400;
        canvas.set_width(size);
        canvas.set_height(size);

        let context = canvas
            .get_context("2d")
            .expect("")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("");

        if expand {
            let res = expand_path(self, size as i32);
            context.draw_path(&res, 3.0);
        } else {
            context.draw_path(self, 3.0);
        }

        let image_data = context
            .get_image_data(0.0, 0.0, size.into(), size.into())
            .expect("");
        let result = image_data
            .data()
            .iter()
            .zip(0..)
            .filter(|(_, index)| index % 4 == 0)
            .map(|x| *x.0)
            .collect::<Vec<_>>();

        result
    }

    fn get_hull(&self) -> Vec<[f64; 2]> {
        let all_points = self
            .clone()
            .into_iter()
            .flatten()
            .map(|x| [x.x(), x.y()])
            .collect::<Vec<_>>();

        graham_scan(&all_points)
    }

    fn get_feature(&self) -> Vec<f64> {
        let hull = self.get_hull();

        let (_, width, height) = minimum_bounding_box(&hull).unwrap_or((vec![], 0.0, 0.0));
        let elongation = (width.max(height) + 1.0) / (width.min(height) + 1.0);

        vec![
            self.get_width(|x| x.x()),
            self.get_width(|x| x.y()),
            elongation,
            polygon_roundness(&hull),
            self.get_pixels(true)
                .into_iter()
                .filter(|x| *x != 0)
                .count() as f64,
            // x: self.path_count() as f64,
            // y: self.point_count() as f64,
        ]
    }
}

fn expand_path<T: Point2DView>(path: &DrawingPaths<T>, size: i32) -> DrawingPaths<[f64; 2]> {
    let points = path
        .clone()
        .into_iter()
        .flatten()
        .map(|x| vec![x.x(), x.y()])
        .collect::<Vec<_>>();

    let bounds = min_max_n_points(&points)
        .map(|(min, max)| Bounds::create(min[0], max[0], min[1], max[1]))
        .unwrap_or(Bounds::create(0.0, 0.0, 0.0, 0.0));

    path.iter()
        .map(|x| {
            x.iter()
                .map(|x| {
                    let new_x = inv_lerp(bounds.left, bounds.right, x.x()) * size as f64;
                    let new_y = inv_lerp(bounds.top, bounds.bottom, x.y()) * size as f64;
                    [new_x, new_y]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn get_feature_names() -> Vec<String> {
    vec![
        "Width".to_owned(),
        "Height".to_owned(),
        "Elongation".to_owned(),
        "Roundness".to_owned(),
        "Complexity".to_owned(),
    ]
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SampleWithFeatures {
    pub sample: Sample,
    pub point: Vec<f64>,
}

impl SampleWithFeatures {
    pub fn create(sample: Sample, point: Vec<f64>) -> Self {
        Self { sample, point }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesData {
    pub feature_names: Vec<String>,
    pub features: Vec<SampleWithFeatures>,
}
