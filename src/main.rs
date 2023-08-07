extern crate sdl2;

use audioswirl::render::render_loop::Renderer;
use sdl2::pixels::Color;

pub fn main() {
    let mut renderer = Renderer::new("audioswirl", 800, 600, 60, Color::BLACK);
    renderer.start();
}
