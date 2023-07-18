use crate::app_state::AppState;
use crate::draw::DrawWithState;
use js_sys::Array;
use std::f64::consts::TAU;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement};

#[derive(Clone)]
pub struct Canvas {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub offset: [f64; 2],
}

impl Canvas {
    pub fn create(document: &Document, id: &str) -> Result<Self, JsValue> {
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let offset = [canvas.width() as f64 / 2.0, canvas.height() as f64 / 2.0];
        let _ = context.translate(offset[0].into(), offset[1].into());

        Ok(Self {
            canvas,
            context,
            offset,
        })
    }
}

impl DrawWithState for Canvas {
    fn draw(&self, _app_state: &AppState) -> Result<(), JsValue> {
        self.context.draw_coordinate_system(&self.offset);
        Ok(())
    }
}

pub trait ContextExt {
    fn draw_coordinate_system(&self, offset: &[f64; 2]);

    fn draw_point(&self, location: &[f64; 2]);
    fn draw_point_with_size(&self, location: &[f64; 2], size: f64);
    fn draw_point_with_size_and_color(&self, location: &[f64; 2], size: f64, color: &str);
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_coordinate_system(&self, offset: &[f64; 2]) {
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

    fn draw_point(&self, location: &[f64; 2]) {
        self.draw_point_with_size(location, 20.0)
    }

    fn draw_point_with_size(&self, location: &[f64; 2], size: f64) {
        self.draw_point_with_size_and_color(location, size, "black")
    }

    fn draw_point_with_size_and_color(&self, location: &[f64; 2], size: f64, color: &str) {
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
}
