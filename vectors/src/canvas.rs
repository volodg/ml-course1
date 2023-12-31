use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::vector::{VectorPolar, VectorXY};
use js_sys::Array;
use std::f64::consts::{PI, TAU};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::log;
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

        let start_point = VectorXY::zero();
        let point_g = VectorXY::new(20.0, 50.0);

        let result_add = app_state.point + point_g;

        self.context.begin_path();
        let array = Array::of2(&JsValue::from(3), &JsValue::from(3));
        self.context.set_line_dash(&array)?;
        self.context.move_to(point_g.x, point_g.y);
        self.context.line_to(result_add.x, result_add.y);
        self.context.line_to(app_state.point.x, app_state.point.y);
        self.context.stroke();
        self.context.set_line_dash(&Array::new())?;

        self.context.draw_arrow(start_point, result_add, "red")?;

        let result_sub = app_state.point - point_g;
        self.context.draw_arrow(start_point, result_sub, "red")?;
        self.context.draw_arrow(point_g, app_state.point, "red")?;

        let normalised_result_sub = result_sub.normalise().scale(50.0);
        self.context
            .draw_arrow(start_point, normalised_result_sub, "red")?;

        log(std::format!("{}", point_g.normalise().dot(app_state.point.normalise())).as_str());

        self.context
            .draw_arrow(start_point, app_state.point, "white")?;
        self.context.draw_arrow(start_point, point_g, "white")?;

        Ok(())
    }
}

trait ContextExt {
    fn draw_coordinate_system(&self, offset: &[f64; 2]) -> Result<(), JsValue>;
    fn draw_point(&self, point: VectorXY, radius: f64, color: &str) -> Result<(), JsValue>;
    fn draw_arrow(&self, start: VectorXY, end: VectorXY, color: &str) -> Result<(), JsValue>;
    fn draw_arrow_with_size(
        &self,
        start: VectorXY,
        end: VectorXY,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue>;
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

    fn draw_point(&self, point: VectorXY, radius: f64, color: &str) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.arc(point.x, point.y, radius, 0.0, TAU)?;
        self.fill();
        Ok(())
    }

    fn draw_arrow(&self, start: VectorXY, end: VectorXY, color: &str) -> Result<(), JsValue> {
        self.draw_arrow_with_size(start, end, color, 20.0)
    }

    fn draw_arrow_with_size(
        &self,
        start: VectorXY,
        end: VectorXY,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue> {
        let polar_point: VectorPolar = (end - start).into();

        let vector1 = VectorPolar::new(polar_point.direction + PI * 0.8, size / 2.0);
        let point1: VectorXY = vector1.into();
        let t1 = point1 + end;

        let vector2 = VectorPolar::new(polar_point.direction - PI * 0.8, size / 2.0);
        let point2: VectorXY = vector2.into();
        let t2 = point2 + end;

        self.begin_path();
        self.move_to(start.x, start.y);
        self.line_to(end.x, end.y);
        self.set_stroke_style(&JsValue::from_str(color));
        self.stroke();

        self.begin_path();
        self.move_to(end.x, end.y);
        self.line_to(t1.x, t1.y);
        self.line_to(t2.x, t2.y);
        self.close_path();
        self.stroke();
        self.set_fill_style(&JsValue::from_str(color));
        self.fill();

        Ok(())
    }
}
