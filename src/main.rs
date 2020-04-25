mod color;
mod matrix;

use color::Color;
use matrix::Vec3;

fn main() {
    let mut pm = PixMap::default();

    let upper_left = Vec3::new(-2.0, 1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    for j in 0..pm.height {
        for i in 0..pm.width {
            let u = i as f32 / pm.width as f32;
            let v = j as f32 / pm.height as f32;

            let ray = Ray::new(Vec3::default(), upper_left + u * horizontal - v * vertical);

            pm.pixels.push(ray.color());
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
            width: 200,
            height: 100,
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
