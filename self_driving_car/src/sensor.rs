use crate::car::Car;
use commons::geometry::{Intersection, Line2D, Point2D, Point2DView};
use commons::math::lerp::lerp;
use std::cell::RefCell;
use std::f64::consts::FRAC_PI_2;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Sensor {
    context: CanvasRenderingContext2d,
    ray_count: usize,
    ray_length: f64,
    ray_spread: f64,
    rays: Vec<Line2D>,
    readings: Vec<Option<Intersection>>,
}

impl Sensor {
    pub fn create(context: CanvasRenderingContext2d) -> Rc<RefCell<Self>> {
        Self::create_with_ray_count(context, 5)
    }

    fn create_with_ray_count(
        context: CanvasRenderingContext2d,
        ray_count: usize,
    ) -> Rc<RefCell<Self>> {
        let result = Self {
            context,
            ray_count,
            ray_length: 150.0,
            ray_spread: FRAC_PI_2,
            rays: vec![],
            readings: vec![],
        };
        Rc::new(RefCell::new(result))
    }

    pub fn update(&mut self, car: &Car, borders: &Vec<Line2D>) {
        self.cast_rays(car);

        self.readings = self
            .rays
            .iter()
            .map(|x| Self::get_reading(x, borders))
            .collect::<Vec<_>>();
    }

    fn get_reading(ray: &Line2D, borders: &Vec<Line2D>) -> Option<Intersection> {
        borders
            .iter()
            .flat_map(|border| ray.get_intersection(border))
            .fold((f64::MAX, None), |acc, el| {
                if el.offset < acc.0 {
                    (el.offset, Some(el))
                } else {
                    acc
                }
            })
            .1
    }

    pub fn cast_rays(&mut self, car: &Car) {
        let start = &car.position;

        self.rays = (0..self.ray_count)
            .map(|i| {
                let ray_angle = lerp(
                    self.ray_spread / 2.0,
                    -self.ray_spread / 2.0,
                    if self.ray_count == 1 {
                        0.5
                    } else {
                        i as f64 / (self.ray_count - 1) as f64
                    },
                ) + car.angle;

                let end = Point2D::create(
                    car.position.x - ray_angle.sin() * self.ray_length,
                    car.position.y - ray_angle.cos() * self.ray_length,
                );

                Line2D {
                    start: start.clone(),
                    end,
                }
            })
            .collect()
    }

    pub fn draw(&self) {
        for (ray, reading) in self.rays.iter().zip(&self.readings) {
            let end = if let Some(reading) = reading {
                &reading.point
            } else {
                &ray.end
            };

            self.context.begin_path();
            self.context.set_line_width(2.0);
            self.context.set_stroke_style(&JsValue::from_str("yellow"));
            self.context.move_to(ray.start.x, ray.start.y);
            self.context.line_to(end.x, end.y);
            self.context.stroke();

            self.context.begin_path();
            self.context.set_line_width(2.0);
            self.context.set_stroke_style(&JsValue::from_str("black"));
            self.context.move_to(ray.end.x, ray.end.y);
            self.context.line_to(end.x, end.y);
            self.context.stroke();
        }
    }
}
