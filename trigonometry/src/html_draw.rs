use std::f64::consts::TAU;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::html::HtmlDom;

pub trait Draw {
    fn draw(&self) -> Result<(), JsValue>;
}

impl Draw for HtmlDom {
    fn draw(&self) -> Result<(), JsValue> {
        let offset = [self.canvas.width() as i32 / 2, self.canvas.height() as i32 / 2];
        let _ = self.context.translate(offset[0].into(), offset[1].into());

        self.context.draw_coordinate_system(&offset);

        let point_a = [0, 0];
        let point_b = [90, 120];
        let point_c = [point_b[0], 0];

        self.context.draw_point(&point_a);
        self.context.draw_text("A", &point_a);
        self.context.draw_point(&point_b);
        self.context.draw_text("B", &point_b);
        self.context.draw_point(&point_c);
        self.context.draw_text("C", &point_c);

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
        let _ = self.arc(location[0].into(), location[1].into(), size as f64 / 2.0, 0.0, TAU);
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
