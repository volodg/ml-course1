pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}
