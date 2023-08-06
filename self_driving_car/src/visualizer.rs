use commons::geometry::{Point2D, Point2DView};
use commons::math::lerp::lerp;
use commons::network::{Level, NeuralNetwork};
use itertools::Itertools;
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Visualizer {}

impl Visualizer {
    pub fn draw_network(
        canvas: &HtmlCanvasElement,
        context: &CanvasRenderingContext2d,
        network: &NeuralNetwork,
    ) {
        let margin = 50.0;
        let left = margin;
        let top = margin;
        let width = canvas.width() as f64 - margin * 2.0;
        let height = canvas.height() as f64 - margin * 2.0;

        Self::draw_level(&context, &network.levels[0], left, top, width, height);
    }

    fn draw_level(
        context: &CanvasRenderingContext2d,
        level: &Level,
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    ) {
        let right = left + width;
        let bottom = top + height;

        let node_radius = 18.0;

        let inputs_size = level.inputs.len();
        let output_size = level.outputs.len();

        level
            .inputs
            .iter()
            .zip(0..)
            .cartesian_product(level.outputs.iter().zip(0..))
            .for_each(|((input, i), (output, j))| {
                context.begin_path();

                let x = Self::get_node_x(i, inputs_size, left, right);
                context.move_to(x, bottom);

                let x = Self::get_node_x(j, output_size, left, right);
                context.line_to(x, top);

                context.set_line_width(2.0);
                context.set_stroke_style(&JsValue::from_str("orange"));

                context.stroke();
            });

        level.inputs.iter().zip(0..).for_each(|(_, index)| {
            let x = Self::get_node_x(index, inputs_size, left, right);

            context.begin_path();
            context.arc(x, bottom, node_radius, 0.0, TAU).expect("");
            context.set_fill_style(&JsValue::from_str("white"));
            context.fill();
        });

        level.outputs.iter().zip(0..).for_each(|(_, index)| {
            let x = Self::get_node_x(index, output_size, left, right);

            context.begin_path();
            context.arc(x, top, node_radius, 0.0, TAU).expect("");
            context.set_fill_style(&JsValue::from_str("white"));
            context.fill();
        });
    }

    fn get_node_x(index: usize, inputs_size: usize, left: f64, right: f64) -> f64 {
        lerp(
            left,
            right,
            if inputs_size == 1 {
                0.5
            } else {
                index as f64 / (inputs_size as f64 - 1.0)
            },
        )
    }
}
