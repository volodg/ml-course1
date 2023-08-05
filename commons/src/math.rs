pub mod lerp;

use crate::geometry::Point2D;
use crate::math::lerp::inv_lerp;

pub trait PointExt {
    fn distance(&self, to: &Self) -> f64;
}

impl PointExt for Point2D {
    fn distance(&self, to: &Self) -> f64 {
        ((self.x - to.x).powf(2.0) + (self.y - to.y).powf(2.0)).sqrt()
    }
}

impl std::ops::Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Bounds {
    pub top_left: Point2D,
    pub bottom_right: Point2D,
}

impl Bounds {
    pub fn create(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        Self {
            top_left: Point2D { x: left, y: top },
            bottom_right: Point2D {
                x: right,
                y: bottom,
            },
        }
    }

    pub fn left(&self) -> f64 {
        self.top_left.x
    }

    pub fn right(&self) -> f64 {
        self.bottom_right.x
    }

    pub fn top(&self) -> f64 {
        self.top_left.y
    }

    pub fn bottom(&self) -> f64 {
        self.bottom_right.y
    }

    pub fn left_mut(&mut self) -> &mut f64 {
        &mut self.top_left.x
    }

    pub fn right_mut(&mut self) -> &mut f64 {
        &mut self.bottom_right.x
    }

    pub fn top_mut(&mut self) -> &mut f64 {
        &mut self.top_left.y
    }

    pub fn bottom_mut(&mut self) -> &mut f64 {
        &mut self.bottom_right.y
    }

    pub fn set_left(&mut self, value: f64) {
        self.top_left.x = value
    }

    pub fn set_right(&mut self, value: f64) {
        self.bottom_right.x = value
    }

    pub fn set_top(&mut self, value: f64) {
        self.top_left.y = value
    }

    pub fn set_bottom(&mut self, value: f64) {
        self.bottom_right.y = value
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

pub fn min_max_n_points(points: &Vec<Vec<f64>>) -> Option<(Vec<f64>, Vec<f64>)> {
    if points.is_empty() {
        return None;
    }

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

    Some((min, max))
}

pub fn normalize_points_to_min_max(points: Vec<Vec<f64>>) -> ((Vec<f64>, Vec<f64>), Vec<Vec<f64>>) {
    match min_max_n_points(&points) {
        Some((min, max)) => {
            let points = normalize_points(&min, &max, points);
            ((min, max), points)
        }
        None => {
            panic!("empty input")
        }
    }
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
    use crate::geometry::get_nearest;
    use binary_heap_plus::BinaryHeap as BinaryHeapExt;

    #[test]
    fn test_nearest_point() {
        let points = [vec![2.0, 2.0], vec![3.0, 3.0], vec![1.0, 1.0]];

        let point = vec![0.0, 0.0];
        let nearest = get_nearest(&point, &points)[0];
        assert_eq!(points[nearest], vec![1.0, 1.0]);

        let point = vec![3.0, 3.0];
        let nearest = get_nearest(&point, &points)[0];
        assert_eq!(points[nearest], vec![3.0, 3.0]);

        let point = vec![2.0, 2.0];
        let nearest = get_nearest(&point, &points)[0];
        assert_eq!(points[nearest], vec![2.0, 2.0])
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
