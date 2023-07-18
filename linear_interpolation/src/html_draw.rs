use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use js_sys::Date;
use std::cell::RefCell;
use std::f64::consts::TAU;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d};

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn v_lerp(a: [f64; 2], b: [f64; 2], t: f64) -> [f64; 2] {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t)]
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

impl DrawWithState for HtmlDom {
    fn draw(&self, _app_state: &AppState) -> Result<(), JsValue> {
        let point_a = [100.0, 300.0];
        let point_b = [400.0, 100.0];

        let context = self.context.clone();
        let canvas = self.canvas.clone();

        let animation_f = Rc::new(RefCell::new(None));
        let animation_f_copy = animation_f.clone();

        *animation_f_copy.borrow_mut() = Some(Closure::new(move || {
            let point_a = point_a.clone();
            let point_b = point_b.clone();

            context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            let sec = Date::now() as f64 / 2000.0;
            let t = sec - sec.floor();
            let point_c = v_lerp(point_a, point_b, t);
            context.draw_dot(&point_c, "");

            request_animation_frame(animation_f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(animation_f_copy.borrow().as_ref().unwrap());

        self.context.draw_dot(&point_a, "A");
        self.context.draw_dot(&point_b, "B");

        Ok(())
    }
}

trait ContextGraphicExt {
    fn draw_dot(&self, location: &[f64; 2], label: &str);
}

impl ContextGraphicExt for CanvasRenderingContext2d {
    fn draw_dot(&self, location: &[f64; 2], label: &str) {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str("white"));
        self.set_stroke_style(&JsValue::from_str("black"));
        _ = self.arc(location[0], location[1], 10.0, 0.0, TAU);
        self.fill();
        self.stroke();
        self.set_fill_style(&JsValue::from_str("black"));
        self.set_text_align("center");
        self.set_text_baseline("middle");
        self.set_font("bold 14px Arial");
        _ = self.fill_text(label, location[0], location[1]);
    }
}
