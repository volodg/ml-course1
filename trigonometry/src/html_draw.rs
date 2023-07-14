use crate::app_state::AppState;
use crate::draw::Draw;
use crate::html::HtmlDom;
use js_sys::Array;
use std::f64::consts::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

impl Draw for HtmlDom {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        let _ = self
            .context
            .translate(self.offset[0].into(), self.offset[1].into());

        self.context.draw_coordinate_system(&self.offset);
        self.redraw(&app_state)?;

        Ok(())
    }

    fn redraw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.context.draw_point(&app_state.point_a);
        self.context.draw_text("A", &app_state.point_a);
        self.context.draw_point(&app_state.point_b);
        self.context.draw_text("B", &app_state.point_b);
        self.context.draw_point(&app_state.point_c);
        self.context.draw_text("C", &app_state.point_c);

        Ok(())
    }
}

trait ContextExt {
    fn draw_coordinate_system(&self, offset: &[i32; 2]);

    fn draw_point(&self, location: &[i32; 2]);
    fn draw_point_with_size(&self, location: &[i32; 2], size: i32);
    fn draw_point_with_size_and_color(&self, location: &[i32; 2], size: i32, color: &str);

    fn draw_text(&self, text: &str, location: &[i32; 2]);
    fn draw_text_with_color(&self, text: &str, location: &[i32; 2], color: &str);
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_coordinate_system(&self, offset: &[i32; 2]) {
        self.begin_path();
        self.move_to((-offset[0]).into(), 0.0);
        self.line_to(offset[0].into(), 0.0);
        self.move_to(0.0, (-offset[1]).into());
        self.line_to(0.0, offset[1].into());

        let array = Array::of2(&JsValue::from(4.0), &JsValue::from(2.0));
        let _ = self.set_line_dash(&array);
        self.set_line_width(1.0);
        self.set_stroke_style(&JsValue::from_str("gray"));
        self.stroke();

        let _ = self.set_line_dash(&Array::new());
    }

    fn draw_point(&self, location: &[i32; 2]) {
        self.draw_point_with_size(location, 20)
    }

    fn draw_point_with_size(&self, location: &[i32; 2], size: i32) {
        self.draw_point_with_size_and_color(location, size, "black")
    }

    fn draw_point_with_size_and_color(&self, location: &[i32; 2], size: i32, color: &str) {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        let _ = self.arc(
            location[0].into(),
            location[1].into(),
            size as f64 / 2.0,
            0.0,
            TAU,
        );
        self.fill();
    }

    fn draw_text(&self, text: &str, location: &[i32; 2]) {
        self.draw_text_with_color(text, location, "white")
    }

    fn draw_text_with_color(&self, text: &str, location: &[i32; 2], color: &str) {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.set_text_align("center");
        self.set_text_baseline("middle");
        self.set_font("bold 13px Courier");
        let _ = self.fill_text(text, location[0].into(), location[1].into());
    }
}
