use std::fmt;
use::std::ops::{Add, Sub, Mul};

use super::traits::VecMath;
use super::vec2::Vec2;
use super::vec3::Vec3;

// // TODO add from trait from Vec3

// used for homogeneous coordinates in 3d space
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl Add<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, other: &Vec4) -> Self::Output {
        Vec4 { 
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z, 
            w: self.w + other.w, 
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, other: &Vec4) -> Self::Output {
        Vec4 { 
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z, 
            w: self.w - other.w, 
        }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, scalar: f64) -> Self::Output {
        self * &scalar
    }
}

impl Mul<&f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, scalar: &f64) -> Self::Output {
        Vec4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl From<Vec3> for Vec4 {
    /// turns any Vec in 3d space to a set of homogeneous coordinates
    fn from(other: Vec3) -> Self {
        Vec4 { 
            x: other.x, 
            y: other.y, 
            z: other.z, 
            w: 1. 
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟨{}, {}, {}, {}⟩", self.x, self.y, self.z, self.w)
    }
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn new_fill(n: f64) -> Self {
        Vec4 { x: n, y: n, z: n, w: n }
    }

    pub fn zero() -> Self {
        Vec4 { x: 0., y: 0., z: 0., w: 0. }
    }
}

impl VecMath for Vec4 {
    fn distance(&self, other: &Self) -> f64 {
        f64::sqrt(
            (self.x - other.x) * (self.x - other.x) + 
            (self.y - other.y) * (self.y - other.y) + 
            (self.z - other.z) * (self.z - other.z) + 
            (self.w - other.w) * (self.w - other.w)
        )
    }

    fn length(&self) -> f64 {
        f64::sqrt(
            (self.x * self.x) + 
            (self.y * self.y) + 
            (self.z * self.z) + 
            (self.w * self.w) 
        )
    }

    fn normalize(&self) -> Self {
        let dist = self.length();
        Vec4 { 
            x: self.x / dist, 
            y: self.y / dist,
            z: self.z / dist,
            w: self.w / dist,
        }
    }

    fn lerp(&self, other: &Self, t: f64) -> Self {
        self + &((other - self) * t)
    }
}