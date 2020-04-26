mod color;
mod matrix;
mod solids;

use color::Color;
use matrix::Vec3;
use solids::Sphere;

fn main() {
    let mut pm = PixMap::default();

    let origin = Vec3::default();
    let aspect_ratio = pm.width as f32 / pm.height as f32;

    let s1 = Sphere::new(Vec3::new(-0.1, 0.1, -1.0), 0.1);
    let s2 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.1);

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(s1));
    objects.push(Box::new(s2));

    let world = Hittables { objects };

    for j in 0..pm.height {
        for i in 0..pm.width {
            // UV coordinates are on a cartesian plane from -1 to 1.
            let u = i as f32 / pm.width as f32 - 0.5;
            let v = j as f32 / pm.height as f32 - 0.5;

            // Decreasing this value will zoom in the view.
            // It is the "depth" of the rendering plane, so decreasing the
            // value essentially pushes the screen further away and our field
            // of view decreases as the frustum narrows.
            let w = -1.0;

            let ray = Ray::new(origin, Vec3::new(u * aspect_ratio, v, w));

            match world.hit(&ray, 0.0, 1000000000000000.0) {
                Some(h) => {
                    let color =
                        0.5 * Vec3::new(h.normal.x() + 1.0, h.normal.y() + 1.0, h.normal.z() + 1.0);
                    pm.pixels.push(color.into());
                }
                None => {
                    pm.pixels.push(ray.color());
                }
            }
        }
    }

    pm.save();
}

struct PixMap {
    pixels: Vec<Color>,
    width: u16,
    height: u16,
}

impl Default for PixMap {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            pixels: Vec::new(),
        }
    }
}

impl PixMap {
    fn save(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for color in &self.pixels {
            println!("{}", color);
        }
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

    pub fn color(&self) -> Color {
        let unit_dir = self.direction().normalize();

        let t = 0.5 * (unit_dir.y() + 1.0);

        let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.0, 0.7, 1.0);

        c.into()
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

struct Hittables {
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
