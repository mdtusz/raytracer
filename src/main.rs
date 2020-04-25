mod color;
mod matrix;

use color::Color;
use matrix::Vec3;

fn main() {
    let mut pm = PixMap::default();

    for i in 0..pm.width {
        for j in 0..pm.height {
            let color = Vec3::new(i as f32 / pm.width as f32, j as f32 / pm.height as f32, 0.2);
            pm.pixels.push(color.into());
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
            width: 512,
            height: 512,
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

struct Ray {
    origin: Vec3,
    vec: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.vec
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.vec * t
    }
}
