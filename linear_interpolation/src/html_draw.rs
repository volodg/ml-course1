use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use commons::math::lerp::lerp;
use js_sys::{Array, Date};
use std::cell::RefCell;
use std::f64::consts::{PI, TAU};
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d};

fn v_lerp(a: [f64; 2], b: [f64; 2], t: f64) -> [f64; 2] {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t)]
}

fn v_lerp_3d(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [
        lerp(a[0], b[0], t),
        lerp(a[1], b[1], t),
        lerp(a[2], b[2], t),
    ]
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

trait HtmlDomExt {
    fn animation(
        &self,
        app_state: Rc<RefCell<AppState>>,
        point_a: [f64; 2],
        point_b: [f64; 2],
    ) -> Result<(), JsValue>;
}

impl HtmlDomExt for HtmlDom {
    fn animation(
        &self,
        app_state: Rc<RefCell<AppState>>,
        point_a: [f64; 2],
        point_b: [f64; 2],
    ) -> Result<(), JsValue> {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        let sec = Date::now() as f64 / 1000.0;
        let t = ((sec * PI).cos() + 1.0) * 0.5;
        let point_c = v_lerp(point_a, point_b, t);

        self.context.draw_dot(&point_c, "")?;
        self.context.draw_dot(&point_a, "A")?;
        self.context.draw_dot(&point_b, "B")?;

        let orange = [230.0, 150.0, 0.0];
        let blue = [0.0, 70.0, 160.0];

        let color = v_lerp_3d(orange, blue, t);

        self.canvas.style().set_property(
            "background-color",
            std::format!("rgb({},{},{})", color[0], color[1], color[2]).as_str(),
        )?;

        let low_frequency = 200.0;
        let high_frequency = 600.0;

        if let Some(oscillator) = &app_state.borrow().oscillator {
            let new_frequency = lerp(low_frequency, high_frequency, t) as f32;
            oscillator.frequency().set_value(new_frequency);
        }

        self.context.set_stroke_style(&JsValue::from_str("white"));
        self.context.set_text_align("center");
        self.context.set_text_baseline("top");
        self.context.set_font("bond 100px Arial");
        let array = Array::of2(&JsValue::from(lerp(50.0, 130.0, t)), &JsValue::from(130.0));
        self.context.set_line_dash(&array)?;
        self.context
            .stroke_text("click for sound", self.canvas.width() as f64 / 2.0, 10.0)?;
        let array = Array::new();
        self.context.set_line_dash(&array)?;
        self.context
            .set_fill_style(&JsValue::from_str("rgba(255,255,255,0.2)"));
        self.context
            .fill_text("click for sound", self.canvas.width() as f64 / 2.0, 10.0)?;

        Ok(())
    }
}

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let point_a = [100.0, 300.0];
        let point_b = [400.0, 100.0];

        let animation_f = Rc::new(RefCell::new(None));
        let animation_f_copy = animation_f.clone();

        let html = self.clone();

        *animation_f_copy.borrow_mut() = Some(Closure::new(move || {
            html.animation(app_state.clone(), point_a.clone(), point_b.clone())
                .expect("");

            request_animation_frame(animation_f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(animation_f_copy.borrow().as_ref().unwrap());

        Ok(())
    }
}

trait ContextGraphicExt {
    fn draw_dot(&self, location: &[f64; 2], label: &str) -> Result<(), JsValue>;
}

impl ContextGraphicExt for CanvasRenderingContext2d {
    fn draw_dot(&self, location: &[f64; 2], label: &str) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str("white"));
        self.set_stroke_style(&JsValue::from_str("black"));
        self.arc(location[0], location[1], 10.0, 0.0, TAU)?;
        self.fill();
        self.stroke();
        self.set_fill_style(&JsValue::from_str("black"));
        self.set_text_align("center");
        self.set_text_baseline("middle");
        self.set_font("bold 14px Arial");
        self.fill_text(label, location[0], location[1])
    }
}
