use crate::math::lerp::{lerp, remap};
use crate::math::Bounds;
use crate::utils::SomeExt;
use binary_heap_plus::BinaryHeap as BinaryHeapExt;
use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

pub trait Point2DView {
    type PointT;

    fn create(x: f64, y: f64) -> Self::PointT;

    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn average(&self, other: &Self) -> Self::PointT {
        Self::create((self.x() + other.x()) / 2.0, (self.y() + other.y()) / 2.0)
    }

    fn multiply(&self, scale: f64) -> Self::PointT {
        Self::create(self.x() * scale, self.y() * scale)
    }
}

impl Point2DView for [f64; 2] {
    type PointT = [f64; 2];

    fn create(x: f64, y: f64) -> Self::PointT {
        [x, y]
    }

    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2DView for Point2D {
    type PointT = Point2D;

    fn create(x: f64, y: f64) -> Self::PointT {
        Point2D { x, y }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

#[derive(Clone)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

pub struct Intersection {
    pub point: Point2D,
    pub offset: f64,
}

impl Line2D {
    pub fn intersect_polygon(&self, polygon: &Vec<Point2D>) -> bool {
        polygon.iter().zip(polygon.iter().cycle().skip(1)).find(|(from, to)| {
            self.get_intersection(&Line2D {
                start: (*from).clone(),
                end: (*to).clone(),
            }).is_some()
        }).is_some()
    }

    pub fn get_intersection(&self, line: &Line2D) -> Option<Intersection> {
        let a = &self.start;
        let b = &self.end;
        let c = &line.start;
        let d = &line.end;

        let t_top = (d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x);
        let u_top = (c.y - a.y) * (a.x - b.x) - (c.x - a.x) * (a.y - b.y);
        let bottom = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);

        if bottom != 0.0 {
            let t = t_top / bottom;
            let u = u_top / bottom;
            if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
                return Intersection {
                    point: Point2D::create(lerp(a.x, b.x, t), lerp(a.y, b.y, t)),
                    offset: t,
                }
                .some();
            }
        }

        return None;
    }

    pub fn get_intersection_unlimited(&self, line: &Line2D) -> (Intersection, f64) {
        let a = &self.start;
        let b = &self.end;
        let c = &line.start;
        let d = &line.end;

        let t_top = (d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x);
        let bottom = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);

        let t = t_top / bottom;
        return (
            Intersection {
                point: Point2D::create(lerp(a.x, b.x, t), lerp(a.y, b.y, t)),
                offset: t,
            },
            bottom,
        );
    }
}

pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        panic!("incompatible points")
    }

    a.iter()
        .zip(b)
        .fold(0.0, |acc, (a, b)| {
            let diff = a - b;
            acc + (diff * diff)
        })
        .sqrt()
}

pub type PointN = Vec<f64>;
pub type PolygonN = Vec<PointN>;

pub fn remap_2d_point(point: &PointN, from: &Bounds, to: &Bounds) -> Point2D {
    Point2D {
        x: remap(from.left, from.right, to.left, to.right, point[0]),
        y: remap(from.top, from.bottom, to.top, to.bottom, point[1]),
    }
}

pub fn get_nearest(point: &PointN, pixel_points: &[PointN]) -> Vec<usize> {
    get_nearest_k(point, pixel_points, 1)
}

pub fn get_nearest_k(point: &[f64], pixel_points: &[PointN], k: usize) -> Vec<usize> {
    let heap_size = 0.max(pixel_points.len() as i64 - k as i64) as usize;

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
                let distance = euclidean_distance(point, pixel);
                if distance < heap.peek().expect("").1 {
                    result.push(index)
                } else {
                    heap.push((index, distance));
                    let min = heap.pop().expect("");
                    result.push(min.0)
                }
            }
        } else {
            let distance = euclidean_distance(point, pixel);
            heap.push((index, distance))
        }
    }

    result
}

pub fn polygon_length(polygon: &Vec<[f64; 2]>) -> f64 {
    let mut result = 0.0;

    for (el, next_el) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
        result += euclidean_distance(el, next_el)
    }

    result
}

pub fn triangle_area(point_a: &PointN, point_b: &PointN, point_c: &PointN) -> f64 {
    let a = euclidean_distance(point_b, point_c);
    let b = euclidean_distance(point_c, point_a);
    let c = euclidean_distance(point_a, point_b);

    let p = (a + b + c) / 2.0;

    (p * (p - a) * (p - b) * (p - c)).sqrt()
}

pub fn polygon_area(polygon: &Vec<[f64; 2]>) -> f64 {
    if polygon.len() == 0 {
        return 0.0;
    }

    let point_a = &polygon[0];

    let iter_1 = polygon.iter().skip(1);
    let iter_2 = polygon.iter().skip(2);

    let mut result = 0.0;

    for (point_b, point_c) in iter_1.zip(iter_2) {
        result += triangle_area(&point_a.to_vec(), &point_b.to_vec(), &point_c.to_vec())
    }

    result
}

pub fn polygon_roundness(polygon: &Vec<[f64; 2]>) -> f64 {
    let length = polygon_length(polygon);
    let area = polygon_area(polygon);
    let radius = length / TAU;
    let cycle_area = PI * radius.powi(2);
    let result = area / cycle_area;

    if result.is_nan() {
        return 0.0;
    }

    result
}

// finds a point with the lowest vertical position (leftmost wins in case of a tie)
fn lowest_point(points: &[[f64; 2]]) -> Option<[f64; 2]> {
    points.iter().fold(None, |lowest, point| match lowest {
        Some(lowest) => {
            if point[1] > lowest[1] {
                return Some(point.clone());
            }

            if point[1] == lowest[1] && point[0] < lowest[0] {
                return Some(point.clone());
            }

            Some(lowest)
        }
        None => Some(point.clone()),
    })
}

// determines p2 relative position to p1-p3. If it is:
// to the right then the result is 1,
// to the left then the result is -1,
// on the line then the result is 0
fn get_orientation(p1: &[f64; 2], p2: &[f64; 2], p3: &[f64; 2]) -> Ordering {
    let val = (p2[1] - p1[1]) * (p3[0] - p2[0]) - (p2[0] - p1[0]) * (p3[1] - p2[1]);
    if val == 0.0 {
        Ordering::Equal
    } else if val > 0.0 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

// orders points in a counter-clockwise relative to the given origin
fn sort_points(origin: &[f64; 2], points: &mut [[f64; 2]]) {
    points.sort_by(|a, b| {
        let orientation = get_orientation(origin, a, b);

        if orientation == Ordering::Equal {
            return euclidean_distance(origin, a).total_cmp(&euclidean_distance(origin, b));
        }

        orientation.reverse()
    })
}

// builds a convex hull (a polygon) using the Graham scan algorithm
// https://en.wikipedia.org/wiki/Graham_scan
pub fn graham_scan(points: &Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    let mut points = points.clone();

    if points.len() < 3 {
        return points;
    }

    let lowest_point = lowest_point(&points).expect("some for non empty input");
    sort_points(&lowest_point, &mut points);

    // initialize the stack with the first three points
    let mut stack = vec![points[0], points[1], points[2]];

    for i in 3..points.len() {
        let mut top = stack.len() - 1;
        // exclude points from the end
        // until adding a new point won't cause a concave
        // so that the resulting polygon will be convex
        while top > 0 && get_orientation(&stack[top - 1], &stack[top], &points[i]).is_le() {
            stack.pop();
            top -= 1;
        }
        // add the point
        stack.push(points[i]);
    }

    stack
}

// builds a box with one of the edges being coincident with the edge
// between hull's points i and j (expected to be neighbors)
#[allow(dead_code)]
fn coincident_box(
    hull: &Vec<[f64; 2]>,
    origin_from: &[f64; 2],
    origin_to: &[f64; 2],
) -> (Vec<[f64; 2]>, f64, f64) {
    if hull.len() < 3 {
        return (hull.clone(), 0.0, 0.0);
    }

    // a difference between two points (vector that connects them)
    fn diff(a: &[f64; 2], b: &[f64; 2]) -> [f64; 2] {
        [a[0] - b[0], a[1] - b[1]]
    }

    // a dot product of two vectors
    fn dot(a: &[f64; 2], b: &[f64; 2]) -> f64 {
        a[0] * b[0] + a[1] * b[1]
    }

    // a length of a vector
    fn len(a: &[f64; 2]) -> f64 {
        (a[0] * a[0] + a[1] * a[1]).sqrt()
    }

    // adds two vectors
    fn add(a: &[f64; 2], b: &[f64; 2]) -> [f64; 2] {
        [a[0] + b[0], a[1] + b[1]]
    }

    // divides a vector by a given magintued
    fn div(a: &[f64; 2], n: f64) -> [f64; 2] {
        [a[0] / n, a[1] / n]
    }

    // builds a unit vector (one having a length of 1) with the same direction as a given one
    // const unit = (a) => div(a, len(a));
    fn unit(a: &[f64; 2]) -> [f64; 2] {
        div(a, len(a))
    }

    // build base vectors for a new system of coordinates
    // where the x-axis is coincident with the i-j edge
    let base_x = unit(&diff(&origin_to, origin_from));
    // and the y-axis is orthogonal (90 degrees rotation counter-clockwise)
    let base_y = [base_x[1], -base_x[0]];

    let mut left = 0.0;
    let mut right = 0.0;
    let mut top = 0.0;
    let mut bottom = 0.0;
    // for every point of a hull
    for point in hull {
        // calculate position relative to the origin
        let n = [point[0] - origin_from[0], point[1] - origin_from[1]];
        // calculate position in new axis (rotate)
        let v = [dot(&base_x, &n), dot(&base_y, &n)];
        // apply trivial logic for calculating the bounding box
        // as rotation is out of consideration at this point
        left = v[0].min(left);
        top = v[1].min(top);
        right = v[0].max(right);
        bottom = v[1].max(bottom);
    }

    // calculate bounding box vertices back in original screen space
    let vertices = vec![
        add(
            &add(&base_x.multiply(left), &base_y.multiply(top)),
            origin_from,
        ),
        add(
            &add(&base_x.multiply(left), &base_y.multiply(bottom)),
            origin_from,
        ),
        add(
            &add(&base_x.multiply(right), &base_y.multiply(bottom)),
            origin_from,
        ),
        add(
            &add(&base_x.multiply(right), &base_y.multiply(top)),
            origin_from,
        ),
    ];

    (vertices, right - left, bottom - top)
}

// determines the minimum (area) bounding box for a given hull (or set of points)
pub fn minimum_bounding_box(hull: &Vec<[f64; 2]>) -> Option<(Vec<[f64; 2]>, f64, f64)> {
    hull.iter()
        .zip(hull.iter().cycle().skip(1))
        .fold(None, |acc, (el, next_el)| {
            if el[0] == next_el[0] && el[1] == next_el[1] {
                return acc;
            }

            let (vertices, width, height) = coincident_box(hull, el, next_el);

            match acc {
                Some(acc) => {
                    if (width * height) < (acc.1 * acc.2) {
                        Some((vertices, width, height))
                    } else {
                        Some(acc)
                    }
                }
                None => Some((vertices, width, height)),
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        coincident_box, euclidean_distance, get_nearest, polygon_area, polygon_length,
    };

    #[test]
    fn test_get_nearest() {
        let point1 = vec![1.0, 3.0];

        assert_eq!(get_nearest(&point1, &[]), vec![]);
    }

    #[test]
    fn test_euclidean_distance() {
        let point1 = [1.0, 3.0];
        let point2 = [5.0, 0.0];

        assert_eq!(euclidean_distance(&point1, &point2), 5.0);
    }

    #[test]
    fn test_cycle_iter() {
        let points = vec![1, 2, 3];
        let pairs = points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(_, _)>>();

        assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 1)]);
    }

    #[test]
    fn test_polygon_length() {
        let point1 = [1.0, 3.0];
        let point2 = [5.0, 0.0];
        let point3 = [1.0, 0.0];

        assert_eq!(polygon_length(&vec![point1, point2, point3]), 12.0);
    }

    #[test]
    fn test_polygon_area() {
        let point1 = [1.0, 3.0];
        let point2 = [5.0, 0.0];
        let point3 = [1.0, 0.0];

        assert_eq!(polygon_area(&vec![point1, point2, point3]), 6.0);
    }

    #[test]
    fn test_coincident_box() {
        let (vertices, width, height) =
            coincident_box(&vec![[0.0, 0.0], [0.0, 1.0]], &[0.0, 0.0], &[0.0, 1.0]);

        assert_eq!(vertices.len(), 2);
        assert_eq!(width, 0.0);
        assert_eq!(height, 0.0);
    }
}
