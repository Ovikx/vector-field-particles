extern crate sdl2;

use sdl2::pixels::Color;
use vecfield_particle_sim::{fields::*, render::simulation::Simulation};

pub fn main() {
    let mut simulation = Simulation::new(
        "vecfield-particle-sim",
        800,
        600,
        60,
        Color::BLACK,
        vline_convergent,
        false,
    );
    simulation.start();
}
