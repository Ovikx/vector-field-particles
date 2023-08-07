use crate::Particle;
use indexmap::IndexMap;

pub struct ParticleCollection {
    pub particles: IndexMap<u32, Particle>,
    pub count: u32,
}

impl ParticleCollection {
    pub fn new() -> Self {
        ParticleCollection {
            particles: IndexMap::new(),
            count: 0,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.insert(self.count, particle);
        self.count += 1;
    }

    pub fn remove_particle(&mut self, particle_id: u32) {
        self.particles.remove(&particle_id);
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
