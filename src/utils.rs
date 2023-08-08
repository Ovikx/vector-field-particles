pub fn magnitude(vector: (f32, f32)) -> f32 {
    (vector.0.powf(2.0) + vector.1.powf(2.0)).sqrt()
}
