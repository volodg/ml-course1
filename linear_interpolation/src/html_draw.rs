use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &AppState) -> Result<(), JsValue> {
        let point_a = [100.0, 200.0];
        let point_b = [400.0, 200.0];

        let point_с = [
            point_a[0] + (point_b[0] - point_a[0]) / 2.0,
            200.0
        ];

        self.context.draw_dot(&point_a, "A");
        self.context.draw_dot(&point_b, "B");
        self.context.draw_dot(&point_с, "C");

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
