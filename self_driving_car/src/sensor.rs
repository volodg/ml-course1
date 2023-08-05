use crate::car::Car;
use commons::geometry::{Point2D, Point2DView};
use commons::math::lerp::lerp;
use commons::math::Bounds;
use std::cell::RefCell;
use std::f64::consts::FRAC_PI_4;
use std::rc::Rc;

pub struct Sensor {
    car: Rc<RefCell<Car>>,
    ray_count: usize,
    ray_length: f64,
    ray_spread: f64,
    rays: Vec<Bounds>,
}

impl Sensor {
    pub fn create(car: Rc<RefCell<Car>>) -> Self {
        Self::create_with_ray_count(car, 3)
    }

    fn create_with_ray_count(car: Rc<RefCell<Car>>, ray_count: usize) -> Self {
        Self {
            car,
            ray_count,
            ray_length: 100.0,
            ray_spread: FRAC_PI_4,
            rays: vec![],
        }
    }

    pub fn update(&mut self) {
        let car = self.car.borrow();
        let start = &car.position;

        self.rays = (0..self.ray_count)
            .map(|i| {
                let ray_angle = lerp(
                    self.ray_spread / 2.0,
                    -self.ray_spread / 2.0,
                    i as f64 / (self.ray_count - 1) as f64,
                );

                let end = Point2D::create(
                    car.position.x - ray_angle.sin() * self.ray_length,
                    car.position.y - ray_angle.cos() * self.ray_length,
                );

                Bounds {
                    top_left: start.clone(),
                    bottom_right: end,
                }
            })
            .collect()
    }
}
