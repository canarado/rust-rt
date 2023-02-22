pub fn clampf(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    }

    if x > max {
        return max
    }

    x
}