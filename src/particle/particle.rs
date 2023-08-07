use crate::Vector2;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub size: u32,
    pub color: Color,
    pub gradient: Vector2,
}

impl Particle {
    pub fn new(x: i32, y: i32, size: u32, color: Color, gradient: Vector2) -> Self {
        Particle {
            x,
            y,
            size,
            color,
            gradient,
        }
    }

    pub fn apply_gradient(&mut self) {
        self.x = (self.x as f32 + self.gradient.x) as i32;
        self.y = (self.y as f32 + self.gradient.y) as i32;
    }
}
