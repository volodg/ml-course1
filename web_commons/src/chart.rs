use crate::chart_models::{Bounds, Options, Point, Sample};
use crate::graphics::{ContextExt, DrawTextParams};
use commons::math::remap;
use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Element, HtmlCanvasElement};

pub struct Chart {
    samples: Vec<Sample>,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    margin: f64,
    transparency: f64,
    pixel_bounds: Bounds,
    data_bounds: Bounds,
    options: Options,
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
            options
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

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        self.draw_axis()?;

        self.context.set_global_alpha(self.transparency);
        self.draw_samples()?;
        self.context.set_global_alpha(1.0);
        Ok(())
    }

    fn draw_axis(&self) -> Result<(), JsValue> {
        self.context.draw_text_with_params(self.options.axis_labels[0].as_str(), &Point {
            x: self.canvas.width() as f64 / 2.0,
            y: self.pixel_bounds.bottom + self.margin / 2.0,
        }, DrawTextParams {
            size: self.margin as u32,
            ..DrawTextParams::default()
        })
    }

    fn draw_samples(&self) -> Result<(), JsValue> {
        for sample in &self.samples {
            let pixel_location = remap_point(&self.data_bounds, &self.pixel_bounds, &sample.point);
            self.context.draw_point(&pixel_location)?;
        }

        Ok(())
    }
}

fn remap_point(from: &Bounds, to: &Bounds, point: &Point) -> Point {
    Point {
        x: remap(from.left, from.right, to.left, to.right, point.x),
        y: remap(from.top, from.bottom, to.top, to.bottom, point.y),
    }
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
                top: 10.0,
                bottom: 2.0,
            }
        );
    }
}
