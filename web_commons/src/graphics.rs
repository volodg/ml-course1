use crate::chart_models::Point;
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait ContextExt {
    fn draw_text(&self, text: &str, location: &Point) -> Result<(), JsValue>;
    fn draw_text_with_align(&self, text: &str, location: &Point, align: &str) -> Result<(), JsValue>;
    fn draw_text_with_align_and_valign(&self, text: &str, location: &Point, align: &str, v_align: &str) -> Result<(), JsValue>;
    fn draw_text_with_align_and_valign_and_size(&self, text: &str, location: &Point, align: &str, v_align: &str, size: u32) -> Result<(), JsValue>;
    fn draw_text_with_align_and_valign_and_size_and_color(&self, text: &str, location: &Point, align: &str, v_align: &str, size: u32, color: &str) -> Result<(), JsValue>;

    fn draw_point(&self, location: &Point) -> Result<(), JsValue>;
    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue>;
    fn draw_point_with_color_and_size(
        &self,
        location: &Point,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue>;
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_text(&self, text: &str, location: &Point) -> Result<(), JsValue> {
        self.draw_text_with_align(text, location, "center")
    }

    fn draw_text_with_align(&self, text: &str, location: &Point, align: &str) -> Result<(), JsValue> {
        self.draw_text_with_align_and_valign(text, location, align, "middle")
    }

    fn draw_text_with_align_and_valign(&self, text: &str, location: &Point, align: &str, v_align: &str) -> Result<(), JsValue> {
        self.draw_text_with_align_and_valign_and_size(text, location, align, v_align, 10)
    }

    fn draw_text_with_align_and_valign_and_size(&self, text: &str, location: &Point, align: &str, v_align: &str, size: u32) -> Result<(), JsValue> {
        self.draw_text_with_align_and_valign_and_size_and_color(text, location, align, v_align, size, "black")
    }

    fn draw_text_with_align_and_valign_and_size_and_color(&self, text: &str, location: &Point, align: &str, v_align: &str, size: u32, color: &str) -> Result<(), JsValue> {
        self.set_text_align(align);
        self.set_text_baseline(v_align);
        self.set_font(std::format!("bold {}px Courier", size).as_str());
        self.set_fill_style(&JsValue::from_str(color));
        self.fill_text(text, location.x, location.y)?;

        Ok(())
    }

    fn draw_point(&self, location: &Point) -> Result<(), JsValue> {
        self.draw_point_with_color(location, "black")
    }

    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue> {
        self.draw_point_with_color_and_size(location, color, 8.0)
    }

    fn draw_point_with_color_and_size(
        &self,
        location: &Point,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.arc(location.x, location.y, size / 2.0, 0.0, TAU)?;
        self.fill();

        Ok(())
    }
}
