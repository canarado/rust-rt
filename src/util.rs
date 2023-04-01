pub fn clampf(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min
    }

    if x > max {
        return max
    }

    x
}

pub fn clamp(i: usize, min: usize, max: usize) -> usize {
    i.max(min).min(max)
}