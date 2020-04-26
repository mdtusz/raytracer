use crate::matrix::Vec3;
use crate::{Hit, Hittable, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;

        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t > min && t < max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;

                let mut hit = Hit::new(t, point, normal, false);
                hit.set_face_normal(ray, normal);

                return Some(hit);
            }

            let t = (-half_b + root) / a;
            if t > min && t < max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;

                let mut hit = Hit::new(t, point, normal, false);
                hit.set_face_normal(ray, normal);

                return Some(hit);
            }
        }

        None
    }
}
