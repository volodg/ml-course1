use crate::app_state::AppState;
use crate::draw::DrawWithState;
use commons::geometry::{average, distance};
use js_sys::Array;
use js_sys::Math::asin;
use std::f64::consts::{PI, TAU};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement};

#[derive(Clone)]
pub struct CanvasChart {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    pub offset: [i32; 2],
}

impl CanvasChart {
    pub fn create(document: &Document, id: &str) -> Result<Self, JsValue> {
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let offset = [canvas.width() as i32 / 2, canvas.height() as i32 / 2];
        let _ = context.translate(offset[0].into(), offset[1].into());

        Ok(Self {
            canvas,
            context,
            offset,
        })
    }
}

impl DrawWithState for CanvasChart {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        let dist_c = distance(&app_state.point_a, &app_state.point_b);
        let dist_a = distance(&app_state.point_b, &app_state.point_c);
        let dist_b = distance(&app_state.point_c, &app_state.point_a);

        self.context.clear_rect(
            (-self.offset[0]).into(),
            (-self.offset[1]).into(),
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        self.context.draw_coordinate_system(&self.offset);

        let sin = dist_a / dist_c;
        let cos = dist_b / dist_c;
        let tan = dist_a / dist_b;
        let theta = asin(sin);

        self.context.draw_text_with_color(
            std::format!("sin = a/c = {:.2}", sin).as_str(),
            &[
                -self.offset[0] / 2,
                (self.offset[1] as f64 * 0.7) as i32,
            ],
            "red",
        );

        self.context.draw_text_with_color(
            std::format!("cos = b/c = {:.2}", cos).as_str(),
            &[
                -self.offset[0] / 2,
                (self.offset[1] as f64 * 0.8) as i32,
            ],
            "blue",
        );

        self.context.draw_text_with_color(
            std::format!("tan = a/b = {:.2}", tan).as_str(),
            &[
                -self.offset[0] / 2,
                (self.offset[1] as f64 * 0.9) as i32,
            ],
            "magenta",
        );

        self.context.draw_text(
            std::format!(
                "θ = a/c = {:.2} ({}°)",
                theta,
                theta.to_degrees().round() as i32
            )
            .as_str(),
            &[
                self.offset[0] / 2,
                (self.offset[1] as f64 * 0.7) as i32,
            ],
        );

        self.context
            .draw_line(&app_state.point_a, &app_state.point_b);
        self.context
            .draw_text("c", &average(&app_state.point_a, &app_state.point_b));
        self.context
            .draw_line_with_color(&app_state.point_a, &app_state.point_c, "blue");
        self.context.draw_text_with_color(
            "b",
            &average(&app_state.point_a, &app_state.point_c),
            "blue",
        );
        self.context
            .draw_line_with_color(&app_state.point_b, &app_state.point_c, "red");
        self.context.draw_text_with_color(
            "a",
            &average(&app_state.point_b, &app_state.point_c),
            "red",
        );

        self.context.draw_text("θ", &app_state.point_a);

        let start = if app_state.point_b[0] > app_state.point_a[0] {
            0.0
        } else {
            PI
        };
        let mut end = if app_state.point_b[1] < app_state.point_c[1] {
            -theta
        } else {
            theta
        };
        if app_state.point_b[0] < app_state.point_a[0] {
            end = PI - end;
        }
        let clockwise = (app_state.point_b[1] < app_state.point_c[1])
            ^ (app_state.point_b[0] > app_state.point_a[0]);

        self.context.draw_angle(start, end, !clockwise);

        Ok(())
    }
}

trait ContextExt {
    fn draw_angle(&self, start: f64, end: f64, clockwise: bool);
    fn draw_line(&self, from: &[i32; 2], to: &[i32; 2]);
    fn draw_line_with_color(&self, from: &[i32; 2], to: &[i32; 2], color: &str);
    fn draw_coordinate_system(&self, offset: &[i32; 2]);

    fn draw_point(&self, location: &[i32; 2]);
    fn draw_point_with_size(&self, location: &[i32; 2], size: i32);
    fn draw_point_with_size_and_color(&self, location: &[i32; 2], size: i32, color: &str);

    fn draw_text(&self, text: &str, location: &[i32; 2]);
    fn draw_text_with_color(&self, text: &str, location: &[i32; 2], color: &str);
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_angle(&self, start: f64, end: f64, clockwise: bool) {
        self.begin_path();
        self.set_stroke_style(&JsValue::from_str("black"));
        self.set_line_width(2.0);
        let _ = self.arc_with_anticlockwise(0.0, 0.0, 20.0, start, end, clockwise);
        self.stroke();
    }

    fn draw_line(&self, from: &[i32; 2], to: &[i32; 2]) {
        self.draw_line_with_color(from, to, "black")
    }

    fn draw_line_with_color(&self, from: &[i32; 2], to: &[i32; 2], color: &str) {
        self.begin_path();
        self.move_to(from[0].into(), from[1].into());
        self.line_to(to[0].into(), to[1].into());

        self.set_line_width(2.0);
        self.set_stroke_style(&JsValue::from_str(color));
        self.stroke();
    }

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
        self.draw_text_with_color(text, location, "black")
    }

    fn draw_text_with_color(&self, text: &str, location: &[i32; 2], color: &str) {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.set_text_align("center");
        self.set_text_baseline("middle");
        self.set_font("bold 18px Courier");
        self.set_stroke_style(&JsValue::from_str("white"));
        self.set_line_width(7.0);
        let _ = self.stroke_text(text, location[0].into(), location[1].into());
        let _ = self.fill_text(text, location[0].into(), location[1].into());
    }
}
