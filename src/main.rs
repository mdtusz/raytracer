use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::{prelude::*, seq::SliceRandom};
use rayon::prelude::*;
use ultraviolet::Vec3;

mod camera;
mod color;
mod materials;
mod matrix;
mod pixmap;
mod ray;
mod shapes;
mod world;

use camera::Camera;
use color::Color;
use materials::Material;
use pixmap::PixMap;
use ray::Ray;
use shapes::Sphere;
use world::World;

fn main() {
    let aa_samples = 2;
    let max_depth = 8;
    let width = 1920;
    let height = 1080;

    let mut pm = PixMap::new(width, height);

    let navy = Material::Lambertian(Vec3::new(0.2, 0.5, 0.8));
    let red = Material::Lambertian(Vec3::new(0.8, 0.2, 0.4));
    let green = Material::Lambertian(Vec3::new(0.2, 0.3, 0.22));
    let mirror = Material::Metal(Vec3::new(0.5, 0.5, 0.5), 0.0);
    let blur_mirror = Material::Metal(Vec3::new(0.21, 0.2, 0.2), 0.3);
    let glass = Material::Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.55);
    let amber = Material::Dielectric(Vec3::new(1.0, 0.8, 0.78), 1.31);

    let s1 = Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, blur_mirror.clone());
    let s2 = Sphere::new(Vec3::new(0.0, 0.0, -10.0), 1.0, mirror.clone());
    let s3 = Sphere::new(Vec3::new(1.0, 0.2, 2.0), 0.4, red.clone());
    let s4 = Sphere::new(Vec3::new(-2.0, 0.5, 0.0), 0.7, glass.clone());
    let s5 = Sphere::new(Vec3::new(-2.0, 0.5, 0.0), -0.6, glass.clone());
    let s6 = Sphere::new(Vec3::new(-2.0, 0.5, 0.0), 0.45, amber.clone());
    let s7 = Sphere::new(Vec3::new(-0.9, 1.1, -7.0), 0.4, green.clone());

    let mut world = World::new();
    world
        .add_object(Box::new(s1))
        .add_object(Box::new(s2))
        .add_object(Box::new(s3))
        .add_object(Box::new(s4))
        .add_object(Box::new(s5))
        .add_object(Box::new(s6))
        .add_object(Box::new(s7));

    let camera = Camera::new(
        Vec3::new(0.0, 2.0, 10.0),
        Vec3::new(0.0, 1.0, 0.0),
        pm.aspect_ratio(),
        2.0 / 2.0,
        10.0,
        0.05,
        0.0,
        1.0,
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
    pixels.shuffle(&mut thread_rng());

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

    pm.save().unwrap();
    println!("Done.");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
        window
            .update_with_buffer(&pm.to_hex(), pm.width as usize, pm.height as usize)
            .unwrap();
    }
}

pub trait Hittable {
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
