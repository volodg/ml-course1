use crate::app_state::AppState;
use crate::draw::DrawWithState;
use js_sys::Array;
use js_sys::Math::hypot;
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
        _ = context.translate(offset[0].into(), offset[1].into())?;

        Ok(Self {
            canvas,
            context,
            offset,
        })
    }
}

impl DrawWithState for Canvas {
    fn draw(&self, app_state: &AppState) -> Result<(), JsValue> {
        self.context.clear_rect(
            -self.offset[0],
            -self.offset[1],
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        self.context.draw_coordinate_system(&self.offset)?;

        self.context.draw_point(app_state.point, 5.0, "white")?;

        let polar_point = to_polar(app_state.point);
        let xy_point = to_xy(polar_point);

        self.context.draw_point(xy_point, 2.0, "red")?;

        self.context.draw_arrow(app_state.point, "white")?;

        Ok(())
    }
}

trait ContextExt {
    fn draw_coordinate_system(&self, offset: &[f64; 2]) -> Result<(), JsValue>;
    fn draw_point(&self, point: [f64; 2], radius: f64, color: &str) -> Result<(), JsValue>;
    fn draw_arrow(&self, point: [f64; 2], color: &str) -> Result<(), JsValue>;
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_coordinate_system(&self, offset: &[f64; 2]) -> Result<(), JsValue> {
        self.begin_path();
        self.move_to((-offset[0]).into(), 0.0);
        self.line_to(offset[0].into(), 0.0);
        self.move_to(0.0, (-offset[1]).into());
        self.line_to(0.0, offset[1].into());

        let array = Array::of2(&JsValue::from(5.0), &JsValue::from(4.0));
        self.set_line_dash(&array)?;
        self.set_line_width(2.0);
        self.set_stroke_style(&JsValue::from_str("red"));
        self.stroke();

        self.set_line_dash(&Array::new())
    }

    fn draw_point(&self, point: [f64; 2], radius: f64, color: &str) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.arc(point[0], point[1], radius, 0.0, TAU)?;
        self.fill();
        Ok(())
    }

    fn draw_arrow(&self, point: [f64; 2], color: &str) -> Result<(), JsValue> {
        self.begin_path();
        self.move_to(0.0, 0.0);
        self.line_to(point[0], point[1]);
        self.set_stroke_style(&JsValue::from_str(color));
        self.stroke();

        Ok(())
    }
}

fn to_xy(point: [f64; 2]) -> [f64; 2] {
    [
        point[0].cos() * point[1],
        point[0].sin() * point[1],
    ]
}

fn to_polar(point: [f64; 2]) -> [f64; 2] {
    [
        direction(point),
        magnitude(point),
    ]
}

fn direction(point: [f64; 2]) -> f64 {
    (point[1]).atan2(point[0])
}

fn magnitude(point: [f64; 2]) -> f64 {
    hypot(point[0], point[1])
}
