use rand::{prelude::*, random};
use ultraviolet::Vec3;

use crate::color::Color;
use crate::shapes::Sphere;
use crate::{Hit, Ray};

#[derive(Clone)]
pub enum Texture {
    Solid(Color),
    Checker(Box<Texture>, Box<Texture>),
    Perlin(PerlinNoise),
}

#[derive(Clone)]
pub struct PerlinNoise {
    size: usize,
    x_permute: Vec<usize>,
    y_permute: Vec<usize>,
    z_permute: Vec<usize>,
    random_values: Vec<f32>,
}

impl PerlinNoise {
    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            x_permute: Self::generate_permutation(size),
            y_permute: Self::generate_permutation(size),
            z_permute: Self::generate_permutation(size),
            random_values: Self::generate_randoms(size),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f32 {
        let mut u = point.x - point.x.floor();
        let mut v = point.y - point.y.floor();
        let mut w = point.z - point.z.floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = 4 * point.x as usize & (self.size - 1);
        let j = 4 * point.y as usize & (self.size - 1);
        let k = 4 * point.z as usize & (self.size - 1);

        self.random_values[self.x_permute[i] ^ self.y_permute[j] ^ self.z_permute[k]]
    }

    fn generate_permutation(count: usize) -> Vec<usize> {
        let mut p = Vec::new();

        for i in 0..count {
            p.push(i);
        }

        for i in (count - 1)..=0 {
            let target = thread_rng().gen_range(0, i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }

        p
    }

    fn generate_randoms(count: usize) -> Vec<f32> {
        let mut randoms = Vec::new();

        for _ in 0..count {
            randoms.push(random());
        }

        randoms
    }
}

impl Texture {
    pub fn value(&self, hit: &Hit) -> Vec3 {
        match self {
            Self::Solid(c) => c.into(),
            Self::Checker(a, b) => {
                let sines = (10.0 * hit.point.x).sin()
                    * (10.0 * hit.point.y).sin()
                    * (10.0 * hit.point.z).sin();
                if sines < 0.0 {
                    a.value(hit)
                } else {
                    b.value(hit)
                }
            }
            Self::Perlin(noise) => Vec3::new(1.0, 1.0, 1.0) * noise.noise(&hit.point),
        }
    }

    pub fn solid(r: u8, g: u8, b: u8) -> Self {
        let color = Color::new(r, g, b);
        Self::Solid(color)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::Solid(Vec3::new(0.5, 0.5, 0.6).into())
    }
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Clone)]
pub enum Material {
    Dielectric(Vec3, f32),
    Metal(Texture, f32),
    Lambertian(Texture),
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Texture::default())
    }
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection> {
        match self {
            Material::Dielectric(albedo, ior) => {
                let eta_ratio = match hit.front_face {
                    true => 1.0 / ior,
                    false => *ior,
                };

                let unit_direction = ray.direction().normalized();

                let cos_theta = -unit_direction.dot(hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let ref_vec = match eta_ratio * sin_theta > 1.0
                    || random::<f32>() < schlick(cos_theta, eta_ratio)
                {
                    true => reflect(unit_direction, hit.normal),
                    false => refract(unit_direction, hit.normal, eta_ratio),
                };

                let ref_out = Reflection {
                    attenuation: *albedo,
                    scatter: Ray::new(hit.point, ref_vec, ray.time()),
                };

                Some(ref_out)
            }
            Material::Metal(albedo, blur) => {
                let reflected = reflect(ray.direction().normalized(), hit.normal);
                let fuzz = blur.max(0.0).min(1.0) * Sphere::unit().random_point_within();
                let scatter = Ray::new(hit.point, reflected + fuzz, ray.time());

                if scatter.direction().dot(hit.normal) > 0.0 {
                    let reflection = Reflection {
                        attenuation: albedo.value(hit),
                        scatter,
                    };

                    Some(reflection)
                } else {
                    None
                }
            }
            Material::Lambertian(albedo) => {
                let scatter_direction = hit.normal + random_point_lambertian();

                let reflection = Reflection {
                    attenuation: albedo.value(hit),
                    scatter: Ray::new(hit.point, scatter_direction, ray.time()),
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

fn refract(uv: Vec3, normal: Vec3, eta_ratio: f32) -> Vec3 {
    let cos_theta = -uv.dot(normal);

    let parallel = eta_ratio * (uv + cos_theta * normal);
    let perpendicular = -(1.0 - parallel.mag_sq()).sqrt() * normal;

    parallel + perpendicular
}

fn schlick(cosine: f32, ior: f32) -> f32 {
    let r = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r + (1.0 - r) * (1.0 - cosine).powi(5)
}

pub struct Reflection {
    pub attenuation: Vec3,
    pub scatter: Ray,
}
