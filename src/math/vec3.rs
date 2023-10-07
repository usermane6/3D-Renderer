use crate::math::vecmath::VecMath;
use std::fmt;
use::std::ops::{Add, Sub, Mul};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z, 
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z, 
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        self * &scalar
    }
}

impl Mul<&f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: &f64) -> Self::Output {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟨{}, {}, {}⟩", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_fill(n: f64) -> Self {
        Vec3 { x: n, y: n, z: n }
    }

    pub fn zero() -> Self {
        Vec3 { x: 0., y: 0., z: 0. }
    }
}

impl VecMath for Vec3 {
    fn distance(&self, other: &Self) -> f64 {
        f64::sqrt(
            (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y) + (self.z - other.z) * (self.z - other.z)
        )
    }

    fn length(&self) -> f64 {
        f64::sqrt(
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z) 
        )
    }

    fn normalize(&self) -> Self {
        let dist = self.length();
        Vec3 { 
            x: self.x / dist, 
            y: self.y / dist,
            z: self.z / dist
        }
    }

    fn lerp(&self, other: &Self, t: f64) -> Self {
        self + &((other - self) * t)
    }
}