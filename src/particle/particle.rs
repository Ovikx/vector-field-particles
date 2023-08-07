use crate::Vector2;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct Particle {
    pub mass: u32,
    pub x: i32,
    pub y: i32,
    pub size: u32,
    pub color: Color,
    pub velocity: Vector2,
}

pub type VectorField = fn((i32, i32)) -> (f32, f32);

impl Particle {
    fn apply_velocity(&mut self) {
        self.x = (self.x as f32 + self.velocity.x) as i32;
        self.y = (self.y as f32 + self.velocity.y) as i32;
    }

    fn apply_field(&mut self, field: VectorField) {
        let acceleration = field((self.x, self.y));
        self.velocity.x += acceleration.0;
        self.velocity.y += acceleration.1;
    }

    pub fn update_position(&mut self, field: VectorField) {
        self.apply_field(field);
        self.apply_velocity();
    }
}
