use rand::random;
use ultraviolet::Vec3;

use crate::materials::Material;
// use crate::matrix::Vec3;
use crate::{Hit, Hittable, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn unit() -> Self {
        Self::new(Vec3::default(), 1.0, Material::default())
    }

    pub fn random_point_within(&self) -> Vec3 {
        let p = Vec3::new(random(), random(), random());

        if self.contains(p) {
            p
        } else {
            self.random_point_within()
        }
    }

    fn contains(&self, point: Vec3) -> bool {
        (point - self.center).mag_sq() < self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;

        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t > min && t < max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;

                let mut hit = Hit::new(t, point, normal, false, self.material.clone());
                hit.set_face_normal(ray, normal);

                return Some(hit);
            }

            let t = (-half_b + root) / a;
            if t > min && t < max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;

                let mut hit = Hit::new(t, point, normal, false, self.material.clone());
                hit.set_face_normal(ray, normal);

                return Some(hit);
            }
        }

        None
    }
}
