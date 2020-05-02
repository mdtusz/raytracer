use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use minifb::{ScaleMode, Window, WindowOptions};
use rand::prelude::*;
use rayon::prelude::*;
use ultraviolet::{Mat4, Vec3, Vec4};

mod color;
mod materials;
mod matrix;
mod shapes;

use color::Color;
use materials::{Material, Scatter};
use shapes::Sphere;

fn main() {
    let navy = Material::Lambertian(Vec3::new(0.2, 0.5, 0.8));
    let red = Material::Lambertian(Vec3::new(0.8, 0.2, 0.4));
    let green = Material::Lambertian(Vec3::new(0.2, 0.4, 0.2));
    let mirror = Material::Metal(Vec3::new(0.5, 0.5, 0.5), 0.0);
    let glass = Material::Dielectric(1.55);

    let s1 = Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, navy.clone());
    let s2 = Sphere::new(Vec3::new(0.0, 0.0, -10.0), 0.5, mirror.clone());
    let s3 = Sphere::new(Vec3::new(1.0, 0.1, 4.0), 0.4, red.clone());
    let s4 = Sphere::new(Vec3::new(-0.4, 0.0, 0.0), 0.4, glass.clone());
    let s5 = Sphere::new(Vec3::new(-0.4, 0.0, 0.0), -0.35, glass.clone());
    let s6 = Sphere::new(Vec3::new(-1.0, 0.0, 3.0), 0.2, green.clone());

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();

    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));
    objects.push(Box::new(s4));
    objects.push(Box::new(s5));
    objects.push(Box::new(s6));

    let aa_samples = 128;
    let max_depth = 512;
    let width = 1920;
    let height = 1080;

    let mut pm = PixMap::new(width, height);

    let world = World { objects };

    let camera = Camera::new(
        Vec3::new(0.0, 1.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        pm.aspect_ratio(),
        1.6 / 2.0,
        (Vec3::new(0.0, 1.0, 10.0) - Vec3::new(0.0, 0.0, 0.0)).mag(),
        0.0,
    );

    let mut options = WindowOptions::default();
    options.resize = true;
    options.scale_mode = ScaleMode::Center;

    let mut window =
        Window::new("Raytracer", pm.width as usize, pm.height as usize, options).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    let mut pixels = Vec::new();
    for j in 0..pm.height {
        for i in 0..pm.width {
            pixels.push((i, j));
        }
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        pixels.into_par_iter().for_each_with(tx, |tx, (x, y)| {
            let mut samples = Vec::new();

            for s in 0..aa_samples {
                let mut sample_i = x as f32;
                let mut sample_j = y as f32;

                if s > 0 {
                    sample_i += random::<f32>() - 0.5;
                    sample_j += random::<f32>() - 0.5;
                }

                // UV coordinates are on a cartesian plane from -1 to 1.
                let u = sample_i / width as f32 - 0.5;
                let v = 1.0 - sample_j / height as f32 - 0.5;

                let ray = camera.get_ray(u, v);

                let sample = ray.trace(&world, max_depth);
                samples.push(sample);
            }

            let c = Color::from_samples(samples);

            tx.send((x, y, c)).expect("wtf");
        });
    });

    let mut now = Instant::now();
    while let Ok((x, y, c)) = rx.recv() {
        pm.update(x, y, c);

        if now.elapsed().as_millis() >= 16 {
            now = Instant::now();
            window
                .update_with_buffer(&pm.to_hex(), pm.width as usize, pm.height as usize)
                .unwrap();
        }
    }

    window
        .update_with_buffer(&pm.to_hex(), pm.width as usize, pm.height as usize)
        .unwrap();

    pm.save().unwrap();
    println!("Done.");

    while window.is_open() {}
}

struct PixMap {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl Default for PixMap {
    fn default() -> Self {
        Self::new(720, 480)
    }
}

impl PixMap {
    fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;

        Self {
            width: width,
            height: height,
            pixels: vec![Color::black(); pixel_count as usize],
        }
    }

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

    fn update(&mut self, x: u32, y: u32, color: Color) {
        let i = x + y * self.width;
        self.pixels[i as usize] = color;
    }

    fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    fn to_hex(&self) -> Vec<u32> {
        self.pixels.iter().map(|c| c.hex()).collect::<Vec<u32>>()
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

        match world.hit(&self, 0.001, f32::INFINITY) {
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
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
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
