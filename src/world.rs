use crate::ray::Ray;
use crate::{Hit, Hittable};

pub struct World {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable + Send + Sync>) -> &mut Self {
        self.objects.push(object);
        self
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<Hit> {
        let (_closest, hit) = self.objects.iter().fold((max, None), |acc, object| {
            match object.hit(ray, min, acc.0) {
                Some(hit) => (hit.t, Some(hit)),
                None => acc,
            }
        });

        Some(hit).flatten()
    }
}
