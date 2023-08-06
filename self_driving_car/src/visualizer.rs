use commons::geometry::{Point2D, Point2DView};
use commons::math::lerp::lerp;
use commons::network::{Level, NeuralNetwork};
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

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

    draw_level(&context, &network.levels[0], left, top, width, height);
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

    level.inputs.iter().zip(0..).for_each(|(input, index)| {
        let x = lerp(
            left,
            right,
            if inputs_size == 1 {
                0.5
            } else {
                index as f64 / (inputs_size as f64 - 1.0)
            },
        );

        context.begin_path();
        context.arc(x, bottom, node_radius, 0.0, TAU).expect("");
        context.set_fill_style(&JsValue::from_str("white"));
        context.fill();
    });

    level.outputs.iter().zip(0..).for_each(|(input, index)| {
        let x = lerp(
            left,
            right,
            if inputs_size == 1 {
                0.5
            } else {
                index as f64 / (inputs_size as f64 - 1.0)
            },
        );

        context.begin_path();
        context.arc(x, top, node_radius, 0.0, TAU).expect("");
        context.set_fill_style(&JsValue::from_str("white"));
        context.fill();
    })
}
