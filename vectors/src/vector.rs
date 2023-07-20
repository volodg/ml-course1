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

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn direction(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn magnitude(&self) -> f64 {
        hypot(self.x, self.y)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn normalise(&self) -> Self {
        self.scale(1.0 / self.magnitude())
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl std::ops::Add<VectorXY> for VectorXY {
    type Output = VectorXY;

    fn add(self, other: VectorXY) -> VectorXY {
        VectorXY {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<VectorXY> for VectorXY {
    type Output = VectorXY;

    fn sub(self, other: VectorXY) -> VectorXY {
        VectorXY {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for VectorXY {
    type Output = VectorXY;

    fn mul(self, other: f64) -> VectorXY {
        VectorXY {
            x: self.x * other,
            y: self.y * other,
        }
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
}

impl From<VectorXY> for VectorPolar {
    fn from(value: VectorXY) -> Self {
        Self {
            direction: value.direction(),
            magnitude: value.magnitude(),
        }
    }
}
