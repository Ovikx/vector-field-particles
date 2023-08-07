extern crate sdl2;

use sdl2::pixels::Color;
use vecfield_particle_sim::render::simulation::Simulation;

pub fn main() {
    let mut simulation = Simulation::new("audioswirl", 800, 600, 60, Color::BLACK);
    simulation.start();
}
