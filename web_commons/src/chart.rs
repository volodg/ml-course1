use std::f64::consts::FRAC_PI_2;
use crate::chart_models::{Bounds, get_data_bounds, Options, Point, Sample};
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
        let data_bounds = get_data_bounds(&samples);

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
            size: (self.margin * 0.6) as u32,
            ..DrawTextParams::default()
        })?;

        self.context.save();
        self.context.translate(self.pixel_bounds.left - self.margin / 2.0, self.canvas.height() as f64 / 2.0)?;
        self.context.rotate(-FRAC_PI_2)?;

        self.context.draw_text_with_params(self.options.axis_labels[1].as_str(), &Point::zero(), DrawTextParams {
            size: (self.margin * 0.6) as u32,
            ..DrawTextParams::default()
        })?;

        self.context.restore();

        Ok(())
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
