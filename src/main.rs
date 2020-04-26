use std::fs::File;
use std::io::Write;

use rand::random;

mod color;
mod matrix;
mod solids;

use color::Color;
use matrix::Vec3;
use solids::Sphere;

fn main() {
    let mut pm = PixMap::default();

    let s1 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let s2 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(s1));
    objects.push(Box::new(s2));

    let world = Hittables { objects };
    let camera = Camera {
        look_at: Vec3::new(0.0, 0.0, -1.0),
        position: Vec3::new(0.0, 0.0, 0.0),
        aspect_ratio: pm.width as f32 / pm.height as f32,
    };

    let aa_samples = 256;
    let max_depth = 256;

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

                // Decreasing this value will zoom in the view.
                // It is the "depth" of the rendering plane, so decreasing the
                // value essentially pushes the screen further away and our field
                // of view decreases as the frustum narrows.
                let w = -0.25;

                let ray = camera.get_ray(u, v, w);

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
            width: 1280,
            height: 760,
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
        let unit_dir = self.direction().normalize();

        let t = 0.5 * (unit_dir.y() + 1.0);

        let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.8, 1.0);

        c
    }

    pub fn trace(&self, world: &Hittables, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }

        let unit_sphere = Sphere::unit();
        match world.hit(&self, 0.0001, f32::INFINITY) {
            Some(hit) => {
                let target = hit.point + hit.normal + unit_sphere.random_point_lambertian();
                let ray = Ray::new(hit.point, target - hit.point);
                0.5 * ray.trace(world, depth - 1)
            }
            None => self.color(),
        }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<Hit>;
}

struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn new(t: f32, point: Vec3, normal: Vec3, front_face: bool) -> Self {
        Self {
            t,
            point,
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

pub struct Hittables {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for Hittables {
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
    look_at: Vec3,
    position: Vec3,
    aspect_ratio: f32,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32, w: f32) -> Ray {
        let origin = Vec3::default();
        Ray::new(origin, Vec3::new(u * self.aspect_ratio, v, w))
    }
}
