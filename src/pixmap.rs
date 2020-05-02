use std::fs::File;
use std::io::Write;

use crate::Color;

pub struct PixMap {
    pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl Default for PixMap {
    fn default() -> Self {
        Self::new(720, 480)
    }
}

impl PixMap {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;

        Self {
            width,
            height,
            pixels: vec![Color::black(); pixel_count as usize],
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
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

    pub fn update(&mut self, x: u32, y: u32, color: Color) {
        let i = x + y * self.width;
        self.pixels[i as usize] = color;
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn to_hex(&self) -> Vec<u32> {
        self.pixels.iter().map(|c| c.hex()).collect::<Vec<u32>>()
    }
}
