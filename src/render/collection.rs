use crate::Particle;

pub struct ParticleCollection {
    pub particles: Vec<Particle>,
}

impl ParticleCollection {
    pub fn new() -> Self {
        ParticleCollection { particles: vec![] }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
