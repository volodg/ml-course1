#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn scale(&self, scale: f64) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    pub fn distance(&self, to: &Self) -> f64 {
        ((self.x - to.x).powf(2.0) + (self.y - to.y).powf(2.0)).sqrt()
    }

    pub fn get_nearest(&self, pixel_points: &[Point]) -> Option<usize> {
        let zero: Option<(f64, usize)> = None;
        pixel_points
            .iter()
            .zip(0..)
            .fold(zero, |acc, (new_point, new_index)| {
                let new_distance = self.distance(new_point);
                let result = acc
                    .map(|(distance, index)| {
                        if new_distance < distance {
                            (new_distance, new_index)
                        } else {
                            (distance, index)
                        }
                    })
                    .unwrap_or((new_distance, new_index));

                Some(result)
            })
            .map(|x| x.1)
    }

    pub fn remap(&self, from: &Bounds, to: &Bounds) -> Point {
        Point {
            x: remap(from.left, from.right, to.left, to.right, self.x),
            y: remap(from.top, from.bottom, to.top, to.bottom, self.y),
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bounds {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

pub fn inv_lerp(a: f64, b: f64, v: f64) -> f64 {
    (v - a) / (b - a)
}

pub fn remap(from_a: f64, from_b: f64, to_a: f64, to_b: f64, v: f64) -> f64 {
    let t = inv_lerp(from_a, from_b, v);
    lerp(to_a, to_b, t)
}

#[cfg(test)]
mod tests {
    use crate::math::Point;

    #[test]
    fn test_nearest_point() {
        let points = [
            Point { x: 2.0, y: 2.0 },
            Point { x: 3.0, y: 3.0 },
            Point { x: 1.0, y: 1.0 },
        ];

        let point = Point::zero();
        let nearest = point.get_nearest(&points).expect("");
        assert_eq!(nearest, Point { x: 1.0, y: 1.0 });

        let point = Point { x: 3.0, y: 3.0 };
        let nearest = point.get_nearest(&points).expect("");
        assert_eq!(nearest, Point { x: 3.0, y: 3.0 });

        let point = Point { x: 2.0, y: 2.0 };
        let nearest = point.get_nearest(&points).expect("");
        assert_eq!(nearest, Point { x: 2.0, y: 2.0 })
    }
}
