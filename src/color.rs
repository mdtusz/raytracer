use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub fn from_samples(samples: Vec<Vec3>) -> Self {
        let summed: Vec3 = samples.iter().fold(Vec3::zero(), |acc, s| *s + acc);
        let scale = 1.0 / samples.len() as f32;

        let r = (summed.x * scale).sqrt().max(0.0).min(1.0);
        let g = (summed.y * scale).sqrt().max(0.0).min(1.0);
        let b = (summed.z * scale).sqrt().max(0.0).min(1.0);

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

    pub fn hex(&self) -> u32 {
        let (r, g, b) = (self.r() as u32, self.g() as u32, self.b() as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn black() -> Self {
        Self(0, 0, 0)
    }
}

impl Into<Color> for Vec3 {
    fn into(self) -> Color {
        let bit_depth = 255.999;

        Color(
            (self.x * bit_depth) as u8,
            (self.y * bit_depth) as u8,
            (self.z * bit_depth) as u8,
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}
