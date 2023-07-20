use js_sys::Math::hypot;

#[derive(Clone, Copy)]
pub struct VectorXY {
    pub x: f64,
    pub y: f64,
}

impl VectorXY {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn direction(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn magnitude(&self) -> f64 {
        hypot(self.x, self.y)
    }
}

impl From<VectorPolar> for VectorXY {
    fn from(value: VectorPolar) -> Self {
        Self {
            x: value.direction.cos() * value.magnitude,
            y: value.direction.sin() * value.magnitude,
        }
    }
}

#[derive(Clone, Copy)]
pub struct VectorPolar {
    pub direction: f64,
    pub magnitude: f64,
}

impl VectorPolar {
    pub fn new(direction: f64, magnitude: f64) -> Self {
        Self {
            direction,
            magnitude,
        }
    }

    // pub fn rotated(self, angle: f64) -> Self {
    //     Self {
    //         direction: self.direction + angle,
    //         ..self
    //     }
    // }
}

impl From<VectorXY> for VectorPolar {
    fn from(value: VectorXY) -> Self {
        Self {
            direction: value.direction(),
            magnitude: value.magnitude(),
        }
    }
}
