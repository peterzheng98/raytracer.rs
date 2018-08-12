extern crate rand;
use self::rand::Rng;
use self::rand::ThreadRng;

use tracer::Vec3;

pub struct Randomizer {
    pub rng: ThreadRng
}

impl Randomizer {
    pub fn random_in_unit_sphere(&mut self) -> Vec3 {
        let mut direction = Vec3::new(1.0, 1.0, 1.0);
        let one_vec = Vec3::new(1.0, 1.0, 1.0);
        while direction.length() >= 1.0 {
            direction = Vec3::new(self.rng.gen_range(0.0, 2.0), self.rng.gen_range(0.0, 2.0), self.rng.gen_range(0.0, 2.0));
            direction = direction - one_vec;
        }
        direction
    }
    pub fn random_in_unit_disk(&mut self) -> Vec3 {
        let mut direction = Vec3::new(1.0, 1.0, 0.0);
        let one_vec = Vec3::new(1.0, 1.0, 0.0);
        while Vec3::dot(direction, direction) >= 1.0 {
            direction = Vec3::new(self.rng.gen_range(0.0, 2.0), self.rng.gen_range(0.0, 2.0), 0.0);
            direction = direction - one_vec;
        }
        direction
    }

    pub fn gen_range(&mut self, a: f64, b: f64) -> f64 {
        self.rng.gen_range(a, b)
    }

    pub fn new() -> Randomizer {
        Randomizer {
            rng: rand::thread_rng()
        }
    }
}
