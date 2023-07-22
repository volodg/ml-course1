use crate::chart_models::{Bounds, Options, Sample};
use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Element, HtmlCanvasElement};

pub struct Chart {
    #[allow(dead_code)]
    samples: Vec<Sample>,
    #[allow(dead_code)]
    canvas: HtmlCanvasElement,
    #[allow(dead_code)]
    context: CanvasRenderingContext2d,
    #[allow(dead_code)]
    margin: f64,
    #[allow(dead_code)]
    transparency: f64,
    #[allow(dead_code)]
    pixel_bounds: Bounds,
    #[allow(dead_code)]
    data_bounds: Bounds,
}

impl Chart {
    pub fn create(
        container: Element,
        samples: Vec<Sample>,
        options: Options,
    ) -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(options.size);
        canvas.set_height(options.size);
        canvas.style().set_property("background-color", "white")?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        container.append_child(&canvas)?;

        let margin = options.size as f64 * 0.1;
        let transparency = 0.5;
        let pixel_bounds = Self::get_pixels_bounds(&canvas, margin);
        let data_bounds = Self::get_data_bounds(&samples);

        Self {
            samples,
            canvas,
            context,
            margin,
            transparency,
            pixel_bounds,
            data_bounds,
        }
        .ok()
    }

    fn get_pixels_bounds(canvas: &HtmlCanvasElement, margin: f64) -> Bounds {
        Bounds {
            left: margin,
            right: canvas.width() as f64 - margin,
            top: margin,
            bottom: canvas.height() as f64 - margin,
        }
    }

    fn get_data_bounds(samples: &[Sample]) -> Bounds {
        let zero_min: Option<f64> = None;
        let zero_max: Option<f64> = None;
        fn min_max(acc: (Option<f64>, Option<f64>), el: f64) -> (Option<f64>, Option<f64>) {
            (
                Some(acc.0.map(|x| x.min(el)).unwrap_or(el)),
                Some(acc.0.map(|x| x.max(el)).unwrap_or(el)),
            )
        }
        let (min_x, max_x) = samples
            .iter()
            .map(|el| el.point.x)
            .fold((zero_min, zero_max), min_max);
        let (min_y, max_y) = samples
            .iter()
            .map(|el| el.point.y)
            .fold((zero_min, zero_max), min_max);
        Bounds {
            left: min_x.expect(""),
            right: max_x.expect(""),
            top: min_y.expect(""),
            bottom: max_y.expect(""),
        }
    }

    pub fn draw(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::chart::Chart;
    use crate::chart_models::{Bounds, Point, Sample};

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

        let result = Chart::get_data_bounds(&samples);
        assert_eq!(
            result,
            Bounds {
                left: 1.0,
                right: 11.0,
                top: 2.0,
                bottom: 10.0,
            }
        );
    }
}
