use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &AppState) -> Result<(), JsValue> {
        let point_a = [100.0, 300.0];
        let point_b = [400.0, 100.0];

        let count = 10;
        for i in 0..count {
            let t = i as f64 / count as f64;
            let point_c = [
                lerp(point_a[0], point_b[0], t),
                lerp(point_a[1], point_b[1], t),
            ];
            self.context.draw_dot(&point_c, std::format!(".{}", i).as_str());
        }

        self.context.draw_dot(&point_a, "A");
        self.context.draw_dot(&point_b, "B");

        Ok(())
    }
}

trait ContextGraphicExt {
    fn draw_dot(&self, location: &[f64; 2], label: &str);
}

impl ContextGraphicExt for CanvasRenderingContext2d {
    fn draw_dot(&self, location: &[f64; 2], label: &str) {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str("white"));
        self.set_stroke_style(&JsValue::from_str("black"));
        _ = self.arc(location[0], location[1], 10.0, 0.0, TAU);
        self.fill();
        self.stroke();
        self.set_fill_style(&JsValue::from_str("black"));
        self.set_text_align("center");
        self.set_text_baseline("middle");
        self.set_font("bold 14px Arial");
        _ = self.fill_text(label, location[0], location[1]);
    }
}
