
pub fn distance(a: &[i32; 2], b: &[i32; 2]) -> f64 {
    let x_diff = (a[0] - b[0]) as f64;
    let y_diff = (a[1] - b[1]) as f64;
    (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt()
}

pub fn average(a: &[i32; 2], b: &[i32; 2]) -> [i32; 2] {
    [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2]
}
