use crate::utils::magnitude;

pub fn inward_field(point: (i32, i32), _window_size: (u32, u32)) -> (f32, f32) {
    let magnitude = magnitude((point.0 as f32, point.1 as f32));
    (-point.0 as f32 / magnitude, -point.1 as f32 / magnitude)
}
