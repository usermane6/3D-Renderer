use std::fmt;
use::std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

use super::traits::VecMath;
use super::vec2::Vec2;
use super::vec3::Vec3;

// used for homogeneous coordinates in 3d space
#[derive(Debug, Clone, Copy)]
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

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
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

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
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

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
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

impl Into<Vec2> for Vec4 {
    fn into(self) -> Vec2 {
        // when decreasing the dim of homogeneous coords you need to account for the z and w disappearing
        Vec2::new(self.x, self.y) * ( 1. / self.z ) * ( 1. / self.w)
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

// implentation of hashing for vectors
// since vec4 contains only floating point values, 
// it cannot be used for hashing
// and since hashing is an easy way to get all the unique points of a set,
// it can be used to get all unique points in a mesh of tris
#[derive(Hash, PartialEq, Eq)]
pub struct Vec4Hash(u64, u64, u64, u64);

impl From<Vec4> for Vec4Hash {
    fn from(value: Vec4) -> Self {
        Vec4Hash(
            value.x.to_bits(),
            value.y.to_bits(),
            value.z.to_bits(),
            value.w.to_bits(),   
        )
    }
}

impl From<Vec4Hash> for Vec4 {
    fn from(value: Vec4Hash) -> Self {
        Vec4 { 
            x: f64::from_bits(value.0),
            y: f64::from_bits(value.1),
            z: f64::from_bits(value.2),
            w: f64::from_bits(value.3),   
        }
    }
}