use crate::matrix::Vec3;

pub struct Color(u8, u8, u8);

impl Color {
    fn r(&self) -> u8 {
        self.0
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn b(&self) -> u8 {
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
