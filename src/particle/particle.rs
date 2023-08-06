use crate::Vector2;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct Particle {
    x: i32,
    y: i32,
    size: u32,
    color: Color,
    gradient: Vector2,
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

    pub fn apply_gradient(&self) -> Self {
        Particle {
            x: (self.x as f32 + self.gradient.x) as i32,
            y: (self.y as f32 + self.gradient.y) as i32,
            size: self.size,
            color: self.color,
            gradient: self.gradient,
        }
    }
}
