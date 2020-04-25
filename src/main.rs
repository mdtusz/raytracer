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

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5);

    for j in 0..pm.height {
        for i in 0..pm.width {
            // UV coordinates are on a cartesian plane from -1 to 1.
            let u = i as f32 / pm.width as f32 - 0.5;
            let v = j as f32 / pm.height as f32 - 0.5;

            // Decreasing this value will zoom in the view.
            // It is the "depth" of the rendering plane, so decreasing the
            // value essentially pushes the screen further away and our field
            // of view decreases as the frustum narrows.
            let w = -0.5;

            let ray = Ray::new(origin, Vec3::new(u * aspect_ratio, v, w));

            if sphere.hit(&ray) {
                pm.pixels.push(Color::new(255, 0, 0));
            } else {
                pm.pixels.push(ray.color());
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
            width: 500,
            height: 500,
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
