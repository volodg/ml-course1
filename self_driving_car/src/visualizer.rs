use commons::math::lerp::lerp;
use commons::network::{Level, NeuralNetwork};
use itertools::Itertools;
use std::f64::consts::TAU;
use js_sys::Array;
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

        let level_height = height / network.levels.len() as f64;
        let levels_count = network.levels.len();

        network.levels.iter().rev().zip(0..).for_each(|(level, index)| {
            let index = levels_count - index - 1;
            let level_top = top + lerp(
                height - level_height,
                0.0,
                if levels_count == 1 {
                    0.5
                } else {
                    index as f64 / (levels_count as f64 - 1.0)
                }
            );

            let is_last = index == levels_count - 1;
            let symbols = if is_last {
                vec!["↑", "←", "→", "↓"]
                // vec!["⬆️", "⬅️", "➡️", "⬇️"]
            } else {
                vec![]
            };

            let array = Array::of2(&JsValue::from(7), &JsValue::from(3));
            context.set_line_dash(&array).expect("");

            Self::draw_level(&context, level, left, level_top, width, level_height, &symbols);
        })
    }

    fn draw_level(
        context: &CanvasRenderingContext2d,
        level: &Level,
        left: f64,
        top: f64,
        width: f64,
        height: f64,
        output_labels: &[&str]
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
            .for_each(|((_, i), (_, j))| {
                context.begin_path();

                let x = Self::get_node_x(i, inputs_size, left, right);
                context.move_to(x, bottom);

                let x = Self::get_node_x(j, output_size, left, right);
                context.line_to(x, top);

                context.set_line_width(2.0);

                context
                    .set_stroke_style(&JsValue::from_str(get_rgba(level.weights[i][j]).as_str()));

                context.stroke();
            });

        level.inputs.iter().zip(0..).for_each(|(input, index)| {
            let x = Self::get_node_x(index, inputs_size, left, right);

            context.begin_path();
            context.arc(x, bottom, node_radius, 0.0, TAU).expect("");
            context.set_fill_style(&JsValue::from_str("black"));
            context.fill();

            context.begin_path();
            context.arc(x, bottom, node_radius * 0.6, 0.0, TAU).expect("");
            context.set_fill_style(&JsValue::from_str(get_rgba(*input).as_str()));
            context.fill();
        });

        level
            .outputs
            .iter()
            .zip(&level.biases)
            .zip(0..)
            .for_each(|((output, bias), index)| {
                let x = Self::get_node_x(index, output_size, left, right);

                context.begin_path();
                context.arc(x, top, node_radius, 0.0, TAU).expect("");
                context.set_fill_style(&JsValue::from_str("black"));
                context.fill();

                context.begin_path();
                context.arc(x, top, node_radius * 0.6, 0.0, TAU).expect("");
                context.set_fill_style(&JsValue::from_str(get_rgba(*output).as_str()));
                context.fill();

                context.begin_path();
                context.set_line_width(2.0);
                context.arc(x, top, node_radius * 0.8, 0.0, TAU).expect("");
                context.set_stroke_style(&JsValue::from_str(get_rgba(*bias).as_str()));
                let array = Array::of2(&JsValue::from(3), &JsValue::from(3));
                context.set_line_dash(&array).expect("");
                context.stroke();
                context.set_line_dash(&Array::new()).expect("");

                if output_labels.len() > 0 {
                    context.begin_path();
                    context.set_text_align("center");
                    context.set_text_baseline("middle");
                    context.set_fill_style(&JsValue::from_str("black"));
                    context.set_stroke_style(&JsValue::from_str("white"));
                    context.set_font(std::format!("{}px Arial", node_radius * 1.5).as_str());
                    context.fill_text(output_labels[index], x, top);
                    context.set_line_width(0.5);
                    context.stroke_text(output_labels[index], x, top);
                }
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

fn get_rgba(value: f64) -> String {
    let red = if value < 0.0 { 0.0 } else { 255.0 };
    let green = red;
    let blue = if value > 0.0 { 0.0 } else { 255.0 };
    let alpha = value.abs();

    std::format!("rgba({red},{green},{blue},{alpha})")
}
