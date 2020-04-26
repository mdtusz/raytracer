use crate::matrix::Vec3;

pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub fn from_samples(samples: Vec<Vec3>) -> Self {
        let summed: Vec3 = samples.iter().sum();
        let scale = 1.0 / samples.len() as f32;

        let r = (summed.x() * scale).sqrt().max(0.0).min(1.0);
        let g = (summed.y() * scale).sqrt().max(0.0).min(1.0);
        let b = (summed.z() * scale).sqrt().max(0.0).min(1.0);

        Vec3::new(r, g, b).into()
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }
}

impl Into<Color> for Vec3 {
    fn into(self) -> Color {
        let bit_depth = 255.999;

        Color(
            (self.x() * bit_depth) as u8,
            (self.y() * bit_depth) as u8,
            (self.z() * bit_depth) as u8,
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}
