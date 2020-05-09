use ultraviolet::Vec3;

use crate::materials::Scatter;
use crate::world::World;
use crate::Hittable;

pub struct Ray {
    origin: Vec3,
    vec: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, vec: Vec3, time: f32) -> Self {
        Ray { origin, vec, time }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.vec
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.vec * t
    }

    pub fn color(&self) -> Vec3 {
        let unit_dir = self.direction().normalized();

        let t = 0.5 * (unit_dir.y + 1.0);

        let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.8, 1.0);

        c
    }

    pub fn trace(&self, world: &World, depth: u32) -> Vec3 {
        if depth <= 0 {
            // Black if we have exceeded the max depth.
            return Vec3::default();
        }

        match world.hit(&self, 0.001, f32::INFINITY) {
            Some(hit) => match hit.material.scatter(&self, &hit) {
                Some(reflection) => {
                    reflection.attenuation * reflection.scatter.trace(world, depth - 1)
                }
                None => Vec3::default(),
            },
            None => self.color(),
        }
    }
}
