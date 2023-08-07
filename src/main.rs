extern crate sdl2;

use audioswirl::render::simulation::Simulation;
use sdl2::pixels::Color;

pub fn main() {
    let mut simulation = Simulation::new("audioswirl", 800, 600, 60, Color::BLACK);
    simulation.start();
}
