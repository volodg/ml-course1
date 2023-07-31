pub fn average(a: &[f64; 2], b: &[f64; 2]) -> [f64; 2] {
    [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0]
}

pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        panic!("incompatible points")
    }

    a.iter()
        .zip(b)
        .fold(0.0, |acc, (a, b)| acc + (a - b).powi(2))
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


#[cfg(test)]
mod tests {
    use crate::geometry::{euclidean_distance, polygon_length};

    #[test]
    fn test_euclidean_distance() {
        let point1 = [1.0, 3.0];
        let point2 = [5.0, 0.0];

        assert_eq!(euclidean_distance(&point1, &point2), 5.0);
    }

    #[test]
    fn test_cycle_iter() {
        let points = vec![1, 2, 3];
        let pairs = points.iter().zip(points.iter().cycle().skip(1)).map(|(a, b)| {
            (*a, *b)
        }).collect::<Vec<(_, _)>>();

        assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 1)]);
    }

    #[test]
    fn test_polygon_length() {
        let point1 = vec![1.0, 3.0];
        let point2 = vec![5.0, 0.0];
        let point3 = vec![1.0, 0.0];

        assert_eq!(polygon_length(&vec![point1, point2, point3]), 12.0);
    }
}
