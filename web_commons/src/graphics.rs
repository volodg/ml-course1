use crate::chart_models::Point;
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait ContextExt {
    fn draw_text(&self, text: &str, location: &Point) -> Result<(), JsValue>;
    fn draw_text_with_params(
        &self,
        text: &str,
        location: &Point,
        params: DrawTextParams,
    ) -> Result<(), JsValue>;

    fn draw_point(&self, location: &Point) -> Result<(), JsValue>;
    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue>;
    fn draw_point_with_color_and_size(
        &self,
        location: &Point,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue>;
}

pub struct DrawTextParams {
    pub align: String,
    pub v_align: String,
    pub size: u32,
    pub color: String,
}

impl Default for DrawTextParams {
    fn default() -> Self {
        Self {
            align: "center".to_owned(),
            v_align: "middle".to_owned(),
            size: 10,
            color: "black".to_owned(),
        }
    }
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_text(&self, text: &str, location: &Point) -> Result<(), JsValue> {
        self.draw_text_with_params(text, location, DrawTextParams::default())
    }

    fn draw_text_with_params(
        &self,
        text: &str,
        location: &Point,
        params: DrawTextParams,
    ) -> Result<(), JsValue> {
        self.set_text_align(&params.align);
        self.set_text_baseline(&params.v_align);
        self.set_font(std::format!("bold {}px Courier", params.size).as_str());
        self.set_fill_style(&JsValue::from_str(params.color.as_str()));
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
