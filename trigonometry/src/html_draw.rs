use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::html::HtmlDom;

pub trait Draw {
    fn draw(&self) -> Result<(), JsValue>;
}

impl Draw for HtmlDom {
    fn draw(&self) -> Result<(), JsValue> {
        let _point_a = [0, 0];
        let point_b = [90, 120];
        let _point_c = [point_b[0], 0];

        self.context.draw_point_with_size(&point_b, 10);

        Ok(())
    }
}

trait ContextExt {
    fn draw_point(&self, location: &[i32; 2]);
    fn draw_point_with_size(&self, location: &[i32; 2], size: i32);
    fn draw_point_with_size_and_color(&self, location: &[i32; 2], size: i32, color: &str);
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_point(&self, location: &[i32; 2]) {
        self.draw_point_with_size(location, 20)
    }

    fn draw_point_with_size(&self, location: &[i32; 2], size: i32) {
        self.draw_point_with_size_and_color(location, size, "black")
    }

    fn draw_point_with_size_and_color(&self, location: &[i32; 2], size: i32, color: &str) {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        let _ = self.arc(location[0] as f64, location[1] as f64, size as f64, 0.0, TAU);
        self.fill();
    }
}
