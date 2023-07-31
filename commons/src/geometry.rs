use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

pub fn average(a: &[f64; 2], b: &[f64; 2]) -> [f64; 2] {
    [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0]
}

pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        panic!("incompatible points")
    }

    (a.iter().zip(b).fold(0.0, |acc, (a, b)| {
        let diff = a - b;
        acc + (diff * diff)
    }) as f64)
        .sqrt()
}

pub type PointN = Vec<f64>;
pub type PolygonN = Vec<PointN>;

pub fn polygon_length(polygon: &PolygonN) -> f64 {
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

pub fn polygon_area(polygon: &PolygonN) -> f64 {
    if polygon.len() == 0 {
        return 0.0;
    }

    let point_a = &polygon[0];

    let iter_1 = polygon.iter().skip(1);
    let iter_2 = polygon.iter().skip(2);

    let mut result = 0.0;

    for (point_b, point_c) in iter_1.zip(iter_2) {
        result += triangle_area(point_a, point_b, point_c)
    }

    result
}

pub fn polygon_roundness(polygon: &PolygonN) -> f64 {
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
fn coincident_box(hull: &Vec<[f64; 2]>, i: usize, j: usize) {
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

    // multiplies a vector by a given magnitude
    // const mult = (a, n) => [a[0] * n, a[1] * n];
    fn mult(a: &[f64; 2], n: f64) -> [f64; 2] {
        [a[0] * n, a[1] * n]
    }
}

/*
geometry.coincidentBox = (hull, i, j) => {
    // divides a vector by a given magintued
    const div = (a, n) => [a[0] / n, a[1] / n];
    // builds a unit vector (one having a length of 1) with the same direction as a given one
    const unit = (a) => div(a, len(a));

    let origin = hull[i];
    // build base vectors for a new system of coordinates
    // where the x-axis is coincident with the i-j edge
    let baseX = unit(diff(hull[j], origin));
    // and the y-axis is orthogonal (90 degrees rotation counter-clockwise)
    let baseY = [baseX[1], -baseX[0]];

    let left = 0;
    let right = 0;
    let top = 0;
    let bottom = 0;
    // for every point of a hull
    for (const p of hull) {
        // calculate position relative to the origin
        const n = [p[0] - origin[0], p[1] - origin[1]];
        // calculate position in new axis (rotate)
        const v = [dot(baseX, n), dot(baseY, n)];
        // apply trivial logic for calculating the bounding box
        // as rotation is out of consideration at this point
        left = Math.min(v[0], left);
        top = Math.min(v[1], top);
        right = Math.max(v[0], right);
        bottom = Math.max(v[1], bottom);
    }

    // calculate bounding box vertices back in original screen space
    const vertices = [
        add(add(mult(baseX, left), mult(baseY, top)), origin),
        add(add(mult(baseX, left), mult(baseY, bottom)), origin),
        add(add(mult(baseX, right), mult(baseY, bottom)), origin),
        add(add(mult(baseX, right), mult(baseY, top)), origin),
    ];

    return {
        vertices,
        width: right - left,
        height: bottom - top,
    };
}
 */

#[cfg(test)]
mod tests {
    use crate::geometry::{euclidean_distance, polygon_area, polygon_length};

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
        let point1 = vec![1.0, 3.0];
        let point2 = vec![5.0, 0.0];
        let point3 = vec![1.0, 0.0];

        assert_eq!(polygon_length(&vec![point1, point2, point3]), 12.0);
    }

    #[test]
    fn test_polygon_area() {
        let point1 = vec![1.0, 3.0];
        let point2 = vec![5.0, 0.0];
        let point3 = vec![1.0, 0.0];

        assert_eq!(polygon_area(&vec![point1, point2, point3]), 6.0);
    }
}
