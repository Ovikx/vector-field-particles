extern crate sdl2;

use sdl2::pixels::Color;
use vecfield_particle_sim::{fields::*, render::simulation::Simulation};

pub fn main() {
    let mut simulation = Simulation::new(
        "vecfield-particle-sim",
        1280,
        720,
        60,
        Color::BLACK,
        y_convergent,
        true,
    );
    simulation.start();
}
