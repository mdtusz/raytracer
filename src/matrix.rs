use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::random;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn random() -> Self {
        Self(random(), random(), random())
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.y() * other.z() - self.x() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(self * other.0, self * other.1, self * other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl<'a> Sum<&'a Self> for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Vec3::default(), |acc, v| *v + acc)
    }
}
