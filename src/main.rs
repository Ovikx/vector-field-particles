extern crate sdl2;

use audioswirl::render::simulation::Simulation;
use sdl2::pixels::Color;

pub fn main() {
    let mut renderer = Simulation::new("audioswirl", 800, 600, 165, Color::BLACK);
    renderer.start();
}
