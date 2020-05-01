use std::fs::File;
use std::io::Write;

use rand::prelude::*;
use ultraviolet::{Mat4, Rotor3, Vec3, Vec4};

mod color;
mod materials;
mod matrix;
mod shapes;

use color::Color;
use materials::{Material, Scatter};
use shapes::Sphere;

fn main() {
    let mut pm = PixMap::default();

    let green = Material::Lambertian(Vec3::new(0.5, 1.0, 0.5));
    let navy = Material::Lambertian(Vec3::new(0.2, 0.5, 0.8));
    let red = Material::Lambertian(Vec3::new(0.8, 0.2, 0.4));
    let green = Material::Lambertian(Vec3::new(0.2, 0.4, 0.2));
    let mirror = Material::Metal(Vec3::new(0.5, 0.5, 0.5), 0.0);
    let glass = Material::Dielectric(1.55);

    let s1 = Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, navy.clone());
    let s2 = Sphere::new(Vec3::new(0.0, 0.0, -10.0), 0.5, mirror.clone());
    let s3 = Sphere::new(Vec3::new(1.0, 0.1, 4.0), 0.4, red.clone());
    let s4 = Sphere::new(Vec3::new(-0.4, 0.0, 0.0), 0.4, glass.clone());
    let s5 = Sphere::new(Vec3::new(-0.4, 0.0, 0.0), -0.38, glass.clone());
    let s6 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.2, green.clone());

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));
    objects.push(Box::new(s4));
    objects.push(Box::new(s5));
    objects.push(Box::new(s6));

    let world = World { objects };

    let camera = Camera::new(
        Vec3::new(0.0, 1.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        pm.aspect_ratio(),
        1.6 / 2.0,
        (Vec3::new(0.0, 1.0, 10.0) - Vec3::new(0.0, 0.0, 0.0)).mag(),
        0.1,
    );

    let aa_samples = 512;
    let max_depth = 8;

    for j in 0..pm.height {
        for i in 0..pm.width {
            let mut samples = Vec::new();

            for s in 0..aa_samples {
                let mut sample_i = i as f32;
                let mut sample_j = j as f32;

                if s > 0 {
                    sample_i += random::<f32>() - 0.5;
                    sample_j += random::<f32>() - 0.5;
                }

                // UV coordinates are on a cartesian plane from -1 to 1.
                let u = sample_i / pm.width as f32 - 0.5;
                let v = 1.0 - sample_j / pm.height as f32 - 0.5;

                let ray = camera.get_ray(u, v);

                let sample = ray.trace(&world, max_depth);
                samples.push(sample);
            }

            let color = Color::from_samples(samples);

            pm.pixels.push(color);
        }
    }

    pm.save().unwrap();
}

struct PixMap {
    pixels: Vec<Color>,
    width: u16,
    height: u16,
}

impl Default for PixMap {
    fn default() -> Self {
        Self {
            width: 720,
            height: 480,
            pixels: Vec::new(),
        }
    }
}

impl PixMap {
    fn save(&self) -> std::io::Result<()> {
        let mut file = File::create("test.ppm")?;
        let mut v: Vec<u8> = Vec::new();

        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        v.extend(header.as_bytes());

        for color in &self.pixels {
            let color_string = format!("{}\n", color);
            v.extend(color_string.as_bytes());
        }

        file.write_all(&v)?;

        Ok(())
    }

    fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

pub struct Ray {
    origin: Vec3,
    vec: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, vec: Vec3) -> Self {
        Ray { origin, vec }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.vec
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
            return Vec3::default();
        }

        match world.hit(&self, 0.0001, f32::INFINITY) {
            Some(hit) => {
                // let target = hit.point + random_point_hemisphere(hit.normal);
                // let ray = Ray::new(hit.point, target - hit.point);
                // 0.5 * ray.trace(world, depth - 1)
                match hit.material.scatter(&self, &hit) {
                    Some(reflection) => {
                        reflection.attenuation * reflection.scatter.trace(world, depth - 1)
                    }
                    None => Vec3::default(),
                }
            }
            None => self.color(),
        }
    }
}

// Diffuse
fn random_point_hemisphere(normal: Vec3) -> Vec3 {
    let p = Sphere::unit().random_point_within();

    if p.dot(normal) > 0.0 {
        p
    } else {
        -p
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<Hit>;
}

pub struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
}

impl Hit {
    pub fn new(t: f32, point: Vec3, normal: Vec3, front_face: bool, material: Material) -> Self {
        Self {
            t,
            point,
            material,
            normal,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        if ray.direction().dot(outward_normal) < 0.0 {
            self.front_face = true;
            self.normal = outward_normal;
        } else {
            self.front_face = false;
            self.normal = -outward_normal;
        }
    }
}

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
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

struct Camera {
    focus_distance: f32,
    lens_radius: f32,
    position: Vec3,
    rotation: Mat4,
    scaling: Vec4,
    w: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        look_at: Vec3,
        aspect: f32,
        fov: f32,
        fd: f32,
        aperture: f32,
    ) -> Self {
        let w = -1.0 / (fov / 2.0).tan();
        let scaling = Vec4::new(aspect, 1.0, 1.0, 1.0);
        let rot = Mat4::look_at(position, look_at, Vec3::new(0.0, 1.0, 0.0));

        Self {
            focus_distance: fd,
            lens_radius: aperture / 2.0,
            position: position,
            rotation: rot.inversed(),
            scaling: scaling,
            w: w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let uvw = Vec4::new(u, v, self.w, 0.0) * self.focus_distance / 2.0 * self.scaling;
        let rd = self.rotation * uvw;

        let dof_offset = random_in_unit_disk() * self.lens_radius;
        // let offset = self.rotation[0] * dof_offset.x + self.rotation[1] * dof_offset.y;
        let offset = self.rotation * Vec4::new(dof_offset.x, dof_offset.y, dof_offset.z, 0.0);

        Ray::new(self.position + offset.xyz(), rd.xyz() - offset.xyz())
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);

        if p.mag() < 1.0 {
            return p;
        }
    }
}
