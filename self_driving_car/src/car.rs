use crate::controls::Controls;
use crate::sensor::Sensor;
use commons::geometry::{Line2D, Point2D};
use commons::utils::{OkExt, SomeExt};
use js_sys::Math::hypot;
use std::cell::RefCell;
use std::f64::consts::PI;
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
    sensor: Option<Rc<RefCell<Sensor>>>,
    polygon: Vec<Point2D>,
    damaged: bool,
}

#[derive(Clone, Copy)]
pub enum ControlType {
    Keys,
    Dummy,
}

impl Car {
    pub fn create(
        context: CanvasRenderingContext2d,
        position: Point2D,
        width: f64,
        height: f64,
        control_type: ControlType,
    ) -> Result<Rc<RefCell<Self>>, JsValue> {
        Self::create_with_max_speed(context, position, width, height, control_type, 3.0)
    }

    pub fn create_with_max_speed(
        context: CanvasRenderingContext2d,
        position: Point2D,
        width: f64,
        height: f64,
        control_type: ControlType,
        max_speed: f64,
    ) -> Result<Rc<RefCell<Self>>, JsValue> {
        let controls = Controls::create(control_type)?;

        let sensor = match control_type {
            ControlType::Keys => Sensor::create(context.clone()).some(),
            ControlType::Dummy => None,
        };

        let car = Rc::new(RefCell::new(Self {
            context,
            position,
            width,
            height,
            speed: 0.0,
            max_speed,
            friction: 0.05,
            acceleration: 0.2,
            angle: 0.0,
            controls,
            sensor,
            polygon: vec![],
            damaged: false,
        }));

        car.ok()
    }

    pub fn update(&mut self, borders: &Vec<Line2D>, _traffic: &[Rc<RefCell<Self>>]) {
        if !self.damaged {
            self.move_by_controls();
            self.polygon = self.create_polygon();

            self.damaged = self.assess_damage(borders);
        }

        if let Some(sensor) = &self.sensor {
            sensor.borrow_mut().update(self, borders);
        }
    }

    fn assess_damage(&self, borders: &Vec<Line2D>) -> bool {
        borders
            .iter()
            .find(|x| x.intersect_polygon(&self.polygon))
            .is_some()
    }

    fn create_polygon(&self) -> Vec<Point2D> {
        let radius = hypot(self.width, self.height) / 2.0;
        let alpha = self.width.atan2(self.height);

        vec![
            Point2D {
                x: self.position.x - (self.angle - alpha).sin() * radius,
                y: self.position.y - (self.angle - alpha).cos() * radius,
            },
            Point2D {
                x: self.position.x - (self.angle + alpha).sin() * radius,
                y: self.position.y - (self.angle + alpha).cos() * radius,
            },
            Point2D {
                x: self.position.x - (PI + self.angle - alpha).sin() * radius,
                y: self.position.y - (PI + self.angle - alpha).cos() * radius,
            },
            Point2D {
                x: self.position.x - (PI + self.angle + alpha).sin() * radius,
                y: self.position.y - (PI + self.angle + alpha).cos() * radius,
            },
        ]
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
        let color = if self.damaged { "gray" } else { "black" };
        self.context.set_fill_style(&JsValue::from_str(color));

        self.context.begin_path();

        self.context.move_to(self.polygon[0].x, self.polygon[0].y);
        for point in &self.polygon {
            self.context.line_to(point.x, point.y);
        }
        self.context.fill();

        if let Some(sensor) = &self.sensor {
            sensor.borrow().draw();
        }

        Ok(())
    }
}
