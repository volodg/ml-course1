use crate::controls::Controls;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Car {
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    controls: Rc<RefCell<Controls>>,
}

impl Car {
    pub fn create(
        context: CanvasRenderingContext2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<Rc<RefCell<Self>>, JsValue> {
        let controls = Controls::create()?;

        Rc::new(RefCell::new(Self {
            context,
            x,
            y,
            width,
            height,
            controls,
        }))
        .ok()
    }

    pub fn draw(&self) {
        self.context.begin_path();
        self.context.rect(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
        );
        self.context.fill();
    }

    pub fn update(&mut self) {
        let controls = self.controls.borrow();

        if controls.forward {
            self.y -= 2.0;
        } else if controls.reverse {
            self.y += 2.0;
        } else if controls.left {
            self.x -= 2.0;
        } else if controls.right {
            self.x += 2.0;
        }
    }
}
