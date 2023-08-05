use crate::controls::Controls;
use crate::sensor::Sensor;
use commons::geometry::{Line2D, Point2D};
use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Car {
    context: CanvasRenderingContext2d,
    pub position: Point2D,
    width: f64,
    height: f64,
    speed: f64,
    max_speed: f64,
    friction: f64,
    acceleration: f64,
    pub angle: f64,
    controls: Rc<RefCell<Controls>>,
    sensor: Rc<RefCell<Sensor>>,
}

impl Car {
    pub fn create(
        context: CanvasRenderingContext2d,
        position: Point2D,
        width: f64,
        height: f64,
    ) -> Result<Rc<RefCell<Self>>, JsValue> {
        let controls = Controls::create()?;
        let sensor = Sensor::create(context.clone());

        let car = Rc::new(RefCell::new(Self {
            context,
            position,
            width,
            height,
            speed: 0.0,
            max_speed: 3.0,
            friction: 0.05,
            acceleration: 0.2,
            angle: 0.0,
            controls,
            sensor: sensor.clone(),
        }));

        car.ok()
    }

    pub fn update(&mut self, borders: &Vec<Line2D>) {
        self.move_by_controls();
        self.sensor.borrow_mut().update(self, borders);
    }

    fn move_by_controls(&mut self) {
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

        let flip = if self.speed != 0.0 {
            if self.speed > 0.0 {
                1.0
            } else {
                -1.0
            }
        } else {
            0.0
        };

        if controls.left {
            self.angle += 0.03 * flip;
        } else if controls.right {
            self.angle -= 0.03 * flip;
        }

        self.position.x -= self.angle.sin() * self.speed;
        self.position.y -= self.angle.cos() * self.speed;
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.save();
        self.context.translate(self.position.x, self.position.y)?;
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

        self.sensor.borrow().draw();

        Ok(())
    }
}
