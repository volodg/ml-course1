use std::f64::consts::TAU;
use crate::chart_models::Point;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait ContextExt {
    fn draw_point(&self, location: &Point) -> Result<(), JsValue>;
    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue>;
    fn draw_point_with_color_and_size(&self, location: &Point, color: &str, size: f64) -> Result<(), JsValue>;
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_point(&self, location: &Point) -> Result<(), JsValue> {
        self.draw_point_with_color(location, "black")
    }

    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue> {
        self.draw_point_with_color_and_size(location, color, 8.0)
    }

    fn draw_point_with_color_and_size(&self, location: &Point, color: &str, size: f64) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.arc(location.x, location.y, size / 2.0, 0.0, TAU)?;
        self.fill();

        Ok(())
    }
}
