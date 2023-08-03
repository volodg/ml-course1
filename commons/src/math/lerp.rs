pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

pub fn inv_lerp(a: f64, b: f64, v: f64) -> f64 {
    if b == a && v == a {
        return v;
    }

    (v - a) / (b - a)
}

pub fn remap(from_a: f64, from_b: f64, to_a: f64, to_b: f64, v: f64) -> f64 {
    let t = inv_lerp(from_a, from_b, v);
    lerp(to_a, to_b, t)
}
