use crate::app_state::AppState;
use crate::canvas::Canvas;
use crate::canvas::ContextExt;
use crate::draw::DrawWithState;
use wasm_bindgen::JsValue;
use web_sys::Document;

#[derive(Clone)]
pub struct CanvasChart {
    pub canvas: Canvas,
}

impl CanvasChart {
    pub fn create(document: &Document, id: &str) -> Result<Self, JsValue> {
        let canvas = Canvas::create(document, id)?;

        Ok(Self { canvas })
    }
}

impl DrawWithState for CanvasChart {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.canvas.draw(app_state)?;

        let chart_scale = self.canvas.offset[1] as f64;

        self.canvas.context.draw_point_with_size_and_color(
            &[((app_state.get_theta() * chart_scale).round()) as i32, (app_state.get_sin() * chart_scale) as i32],
            2,
            "red"
        );

        self.canvas.context.draw_point_with_size_and_color(
            &[((app_state.get_theta() * chart_scale).round()) as i32, (app_state.get_cos() * chart_scale) as i32],
            2,
            "blue"
        );

        self.canvas.context.draw_point_with_size_and_color(
            &[((app_state.get_theta() * chart_scale).round()) as i32, (app_state.get_tan() * chart_scale) as i32],
            2,
            "magenta"
        );

        Ok(())
    }
}
