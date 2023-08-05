use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use commons::geometry::{Point2D, Point2DView};
use commons::math::lerp::lerp;
use std::cell::RefCell;
use std::f64::consts::TAU;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::animations::animate_with_callback;
use web_commons::log;
use web_sys::CanvasRenderingContext2d;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        log("draw me");
        self.canvas
            .set_width(self.window.inner_width().expect("").as_f64().unwrap() as u32);
        self.canvas
            .set_height(self.window.inner_height().expect("").as_f64().unwrap() as u32);

        let t = Box::new(RefCell::new(-1.0));

        let canvas = self.canvas.clone();
        let context = self.context.clone();
        animate_with_callback(move || {
            context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            let a = Point2D::create(200.0, 150.0);
            let b = Point2D::create(150.0, 250.0);
            let c = Point2D::create(50.0, 100.0);
            let d = Point2D::create(250.0, 200.0);

            context.begin_path();
            context.move_to(a.x, a.y);
            context.line_to(b.x, b.y);

            context.move_to(c.x, c.y);
            context.line_to(d.x, d.y);
            context.stroke();

            context.draw_dot(&a, "A", false)?;
            context.draw_dot(&b, "B", false)?;
            context.draw_dot(&c, "C", false)?;
            context.draw_dot(&d, "D", false)?;

            let t_val = *t.borrow();
            log(std::format!("t_val: {}", t_val).as_str());

            let m = Point2D {
                x: lerp(a.x, b.x, t_val),
                y: lerp(a.y, b.y, t_val),
            };

            let is_red = t_val < 0.0 || t_val > 1.0;
            context.draw_dot(&m, "M", is_red)?;

            *t.borrow_mut() += 0.005;

            Ok(())
        });

        Ok(())
    }
}

trait ContextExt {
    fn draw_dot(&self, point: &Point2D, text: &str, is_red: bool) -> Result<(), JsValue>;
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_dot(&self, point: &Point2D, text: &str, is_red: bool) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(if is_red { "red" } else { "white" }));
        self.arc(point.x, point.y, 10.0, 0.0, TAU)?;
        self.fill();
        self.stroke();
        self.set_text_align("center");
        self.set_text_baseline("middle");
        self.set_font("bold 14px Arial");
        self.set_fill_style(&JsValue::from_str("black"));
        self.fill_text(text, point.x, point.y)
    }
}
