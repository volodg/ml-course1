use crate::app_state::AppState;
use crate::canvas::Canvas;
use crate::draw::DrawWithState;
use commons::geometry::average;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Document};

#[derive(Clone)]
pub struct CanvasGraphic {
    pub canvas: Canvas,
}

impl CanvasGraphic {
    pub fn create(document: &Document, id: &str) -> Result<Self, JsValue> {
        let canvas = Canvas::create(document, id)?;

        Ok(Self { canvas })
    }
}

impl DrawWithState for CanvasGraphic {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.canvas.context.clear_rect(
            (-self.canvas.offset[0]).into(),
            (-self.canvas.offset[1]).into(),
            self.canvas.canvas.width().into(),
            self.canvas.canvas.height().into(),
        );

        self.canvas.draw(app_state)?;

        self.canvas.context.draw_text_with_color(
            std::format!("sin = a/c = {:.2}", app_state.get_sin()).as_str(),
            &[-self.canvas.offset[0] / 2.0, self.canvas.offset[1] * 0.7],
            "red",
        );

        self.canvas.context.draw_text_with_color(
            std::format!("cos = b/c = {:.2}", app_state.get_cos()).as_str(),
            &[-self.canvas.offset[0] / 2.0, self.canvas.offset[1] * 0.8],
            "blue",
        );

        self.canvas.context.draw_text_with_color(
            std::format!("tan = a/b = {:.2}", app_state.get_tan()).as_str(),
            &[-self.canvas.offset[0] / 2.0, self.canvas.offset[1] * 0.9],
            "magenta",
        );

        self.canvas.context.draw_text(
            std::format!(
                "θ = a/c = {:.2} ({}°)",
                app_state.get_theta(),
                app_state.get_theta().to_degrees().round() as i32
            )
            .as_str(),
            &[self.canvas.offset[0] / 2.0, self.canvas.offset[1] * 0.7],
        );

        self.canvas
            .context
            .draw_line(&app_state.get_point_a(), &app_state.get_point_b());
        self.canvas.context.draw_text(
            "c",
            &average(&app_state.get_point_a(), &app_state.get_point_b()),
        );
        self.canvas.context.draw_line_with_color(
            &app_state.get_point_a(),
            &app_state.get_point_c(),
            "blue",
        );
        self.canvas.context.draw_text_with_color(
            "b",
            &average(&app_state.get_point_a(), &app_state.get_point_c()),
            "blue",
        );
        self.canvas.context.draw_line_with_color(
            &app_state.get_point_b(),
            &app_state.get_point_c(),
            "red",
        );
        self.canvas.context.draw_text_with_color(
            "a",
            &average(&app_state.get_point_b(), &app_state.get_point_c()),
            "red",
        );

        self.canvas.context.draw_text("θ", &app_state.get_point_a());

        self.canvas
            .context
            .draw_angle(AppState::get_dist_c(), app_state.get_theta());

        Ok(())
    }
}

trait ContextGraphicExt {
    fn draw_angle(&self, radius: f64, end: f64);
    fn draw_line(&self, from: &[f64; 2], to: &[f64; 2]);
    fn draw_line_with_color(&self, from: &[f64; 2], to: &[f64; 2], color: &str);

    fn draw_text(&self, text: &str, location: &[f64; 2]);
    fn draw_text_with_color(&self, text: &str, location: &[f64; 2], color: &str);
}

impl ContextGraphicExt for CanvasRenderingContext2d {
    fn draw_angle(&self, radius: f64, end: f64) {
        self.begin_path();
        self.set_stroke_style(&JsValue::from_str("black"));
        self.set_line_width(2.0);
        let _ = self.arc(0.0, 0.0, radius, 0.0, end);
        self.stroke();
    }

    fn draw_line(&self, from: &[f64; 2], to: &[f64; 2]) {
        self.draw_line_with_color(from, to, "black")
    }

    fn draw_line_with_color(&self, from: &[f64; 2], to: &[f64; 2], color: &str) {
        self.begin_path();
        self.move_to(from[0].into(), from[1].into());
        self.line_to(to[0].into(), to[1].into());

        self.set_line_width(2.0);
        self.set_stroke_style(&JsValue::from_str(color));
        self.stroke();
    }

    fn draw_text(&self, text: &str, location: &[f64; 2]) {
        self.draw_text_with_color(text, location, "black")
    }

    fn draw_text_with_color(&self, text: &str, location: &[f64; 2], color: &str) {
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
