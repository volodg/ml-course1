pub mod lerp;

use crate::math::lerp::{inv_lerp, remap};
use binary_heap_plus::BinaryHeap as BinaryHeapExt;
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait PointExt {
    fn scale(&self, scale: f64) -> Self;
    fn distance(&self, to: &Self) -> f64;
    fn get_nearest(&self, pixel_points: &[Point]) -> Vec<usize>;
    fn get_nearest_k(&self, pixel_points: &[Point], k: usize) -> Vec<usize>;
    fn remap(&self, from: &Bounds, to: &Bounds) -> Point;
}

impl PointExt for Point {
    fn scale(&self, scale: f64) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    fn distance(&self, to: &Self) -> f64 {
        ((self.x - to.x).powf(2.0) + (self.y - to.y).powf(2.0)).sqrt()
    }

    fn get_nearest(&self, pixel_points: &[Point]) -> Vec<usize> {
        self.get_nearest_k(pixel_points, 1)
    }

    fn get_nearest_k(&self, pixel_points: &[Point], k: usize) -> Vec<usize> {
        let heap_size = 0.max(pixel_points.len() - k);

        let mut heap =
            BinaryHeapExt::with_capacity_by(heap_size, |a: &(usize, f64), b: &(usize, f64)| {
                b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
            });

        let mut result = vec![];

        for (pixel, index) in pixel_points.iter().zip(0..) {
            if heap.len() == heap_size {
                if heap_size == 0 {
                    result.push(index)
                } else {
                    let distance = self.distance(pixel);
                    if distance < heap.peek().expect("").1 {
                        result.push(index)
                    } else {
                        heap.push((index, distance));
                        let min = heap.pop().expect("");
                        result.push(min.0)
                    }
                }
            } else {
                let distance = self.distance(pixel);
                heap.push((index, distance))
            }
        }

        result
    }

    fn remap(&self, from: &Bounds, to: &Bounds) -> Point {
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

impl Bounds {
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }
}

pub trait Min {
    fn min_v(self, other: Self) -> Self;
}

pub trait Max {
    fn max_v(self, other: Self) -> Self;
}

impl Min for f64 {
    fn min_v(self, other: Self) -> Self {
        self.min(other)
    }
}

impl Max for f64 {
    fn max_v(self, other: Self) -> Self {
        self.max(other)
    }
}

impl Min for i64 {
    fn min_v(self, other: Self) -> Self {
        self.min(other)
    }
}

impl Max for i64 {
    fn max_v(self, other: Self) -> Self {
        self.max(other)
    }
}

// TODO use https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html#method.minmax
pub fn min_max<NUM: Min + Max + Copy>(
    (acc_min, acc_max): (Option<NUM>, Option<NUM>),
    el: NUM,
) -> (NUM, NUM) {
    (
        acc_min.map(|x| x.min_v(el)).unwrap_or(el),
        acc_max.map(|x| x.max_v(el)).unwrap_or(el),
    )
}

pub fn normalize_points_to_min_max(points: Vec<Vec<f64>>) -> ((Vec<f64>, Vec<f64>), Vec<Vec<f64>>) {
    let mut min = points[0].clone();
    let mut max = points[0].clone();

    let dimensions = points[0].len();
    for i in 1..points.len() {
        for j in 0..dimensions {
            let value = points[i][j];
            min[j] = min[j].min(value);
            max[j] = max[j].max(value);
        }
    }

    let points = normalize_points(&min, &max, points);
    ((min, max), points)
}

pub fn normalize_points(min: &Vec<f64>, max: &Vec<f64>, points: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    points
        .into_iter()
        .map(|row| {
            row.into_iter()
                .zip(0..)
                .map(|(v, i)| inv_lerp(min[i], max[i], v))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::math::{Point, PointExt};
    use binary_heap_plus::BinaryHeap as BinaryHeapExt;

    #[test]
    fn test_nearest_point() {
        let points = [
            Point { x: 2.0, y: 2.0 },
            Point { x: 3.0, y: 3.0 },
            Point { x: 1.0, y: 1.0 },
        ];

        let point = Point::default();
        let nearest = point.get_nearest(&points)[0];
        assert_eq!(points[nearest], Point { x: 1.0, y: 1.0 });

        let point = Point { x: 3.0, y: 3.0 };
        let nearest = point.get_nearest(&points)[0];
        assert_eq!(points[nearest], Point { x: 3.0, y: 3.0 });

        let point = Point { x: 2.0, y: 2.0 };
        let nearest = point.get_nearest(&points)[0];
        assert_eq!(points[nearest], Point { x: 2.0, y: 2.0 })
    }

    #[test]
    fn test_get_nearest_k() {
        let mut heap = BinaryHeapExt::with_capacity_by(10, |a: &i32, b: &i32| b.cmp(a));
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert_eq!(&1, heap.peek().expect(""))
    }
}
