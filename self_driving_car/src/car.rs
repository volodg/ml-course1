use crate::controls::Controls;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_commons::log;

pub struct Car {
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
    max_speed: f64,
    friction: f64,
    acceleration: f64,
    angle: f64,
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
            speed: 0.0,
            max_speed: 3.0,
            friction: 0.05,
            acceleration: 0.2,
            angle: 0.0,
            controls,
        }))
        .ok()
    }

    pub fn update(&mut self) {
        let controls = self.controls.borrow();

        if controls.forward {
            self.speed += self.acceleration;
            self.speed = self.speed.min(self.max_speed);
        } else if controls.reverse {
            self.speed -= self.acceleration;
            self.speed = self.speed.max(-self.max_speed / 2.0);
        }

        if self.speed > 0.0 {
            self.speed -= self.friction;
            self.speed = self.speed.max(0.0);
        } else if self.speed < 0.0 {
            self.speed += self.friction;
            self.speed = self.speed.min(0.0);
        }

        if controls.left {
            self.angle += 0.03;
        } else if controls.right {
            self.angle -= 0.03;
        }

        log(std::format!("speed: {:?}", self.speed).as_str());
        self.x -= self.angle.sin() * self.speed;
        self.y -= self.angle.cos() * self.speed;
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.save();
        self.context.translate(self.x, self.y)?;
        self.context.rotate(-self.angle)?;

        self.context.begin_path();
        self.context.rect(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width,
            self.height,
        );

        self.context.fill();
        self.context.restore();

        Ok(())
    }
}
