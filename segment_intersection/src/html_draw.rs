use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use commons::geometry::{Point2D, Point2DView};
use web_commons::log;

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        log("draw me");
        self.canvas.set_width(self.window.inner_width().expect("").as_f64().unwrap() as u32);
        self.canvas.set_height(self.window.inner_height().expect("").as_f64().unwrap() as u32);

        let a = Point2D::create(200.0, 150.0);
        let b = Point2D::create(150.0, 250.0);
        let c = Point2D::create(50.0, 100.0);
        let d = Point2D::create(250.0, 200.0);

        self.context.begin_path();
        self.context.move_to(a.x, a.y);
        self.context.line_to(b.x, b.y);

        self.context.move_to(c.x, c.y);
        self.context.line_to(d.x, d.y);
        self.context.stroke();

        Ok(())
    }
}
