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

pub type VectorField = fn(point: (i32, i32), window_size: (u32, u32)) -> (f32, f32);

fn is_bounded(pos_x: i32, pos_y: i32, window_x: u32, window_y: u32) -> bool {
    pos_x > 0 && pos_x < window_x as i32 && pos_y > 0 && pos_y < window_y as i32
}

impl Particle {
    fn apply_velocity(&mut self) {
        self.x = (self.x as f32 + self.velocity.x) as i32;
        self.y = (self.y as f32 + self.velocity.y) as i32;
    }

    /// Applies the vector field (in N) to the particle based on its position relative to the center of the screen
    fn apply_field(
        &mut self,
        field: VectorField,
        factor: f32,
        window_size: (u32, u32),
        bounded: bool,
    ) {
        let (shifted_x, shifted_y) = (
            self.x - (window_size.0 / 2) as i32,
            self.y - (window_size.1 / 2) as i32,
        );

        let force: (f32, f32) = match bounded {
            true => {
                if is_bounded(self.x, self.y, window_size.0, window_size.1) {
                    field((shifted_x, shifted_y), window_size)
                } else {
                    let magnitude = ((shifted_x.pow(2) + shifted_y.pow(2)) as f32).sqrt();
                    (-shifted_x as f32 / magnitude, -shifted_y as f32 / magnitude)
                }
            }
            false => field((shifted_x, shifted_y), window_size),
        };

        self.velocity.x += force.0 / self.mass as f32 * factor;
        self.velocity.y += force.1 / self.mass as f32 * factor;
    }

    /// Abstraction to update the position of the particle given the window information
    pub fn update_position(
        &mut self,
        field: VectorField,
        factor: f32,
        window_size: (u32, u32),
        bounded: bool,
    ) {
        self.apply_field(field, factor, window_size, bounded);
        self.apply_velocity();
    }
}
