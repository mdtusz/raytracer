use rand::random;

use crate::matrix::Vec3;
use crate::shapes::Sphere;
use crate::{Hit, Ray};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Clone)]
pub enum Material {
    Metal(Vec3, f32),
    Lambertian(Vec3),
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Vec3::new(0.5, 0.5, 0.6))
    }
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection> {
        match self {
            Material::Metal(albedo, blur) => {
                let reflected = reflect(ray.direction().normalize(), hit.normal);
                let fuzz = blur.max(0.0).min(1.0) * Sphere::unit().random_point_within();
                let scatter = Ray::new(hit.point, reflected + fuzz);

                if scatter.direction().dot(hit.normal) > 0.0 {
                    let reflection = Reflection {
                        attenuation: *albedo,
                        scatter: scatter,
                    };

                    Some(reflection)
                } else {
                    None
                }
            }
            Material::Lambertian(albedo) => {
                let scatter_direction = hit.normal + random_point_lambertian();

                let reflection = Reflection {
                    attenuation: *albedo,
                    scatter: Ray::new(hit.point, scatter_direction),
                };

                Some(reflection)
            }
        }
    }
}

// Diffuse
fn random_point_lambertian() -> Vec3 {
    let a = random::<f32>() * 2.0 * std::f32::consts::PI;
    let z = random::<f32>() * 2.0 - 1.0;
    let r = (1.0 - z * z).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}

pub struct Reflection {
    pub attenuation: Vec3,
    pub scatter: Ray,
}
