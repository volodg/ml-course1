use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, window};
use commons::utils::OkExt;

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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

pub struct Chart {
    #[allow(dead_code)]
    samples: Vec<Sample>,
    #[allow(dead_code)]
    canvas: HtmlCanvasElement,
    #[allow(dead_code)]
    context: CanvasRenderingContext2d,
}

impl Chart {
    pub fn create(_element: Element, samples: Vec<Sample>, options: Options) -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(options.size);
        canvas.set_height(options.size);

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Self {
            samples,
            canvas,
            context
        }.ok()
    }
}
