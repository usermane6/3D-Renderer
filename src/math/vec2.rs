use crate::math::traits::VecMath;
use std::fmt;
use::std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};


#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Self::Output {
        Vec2 { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Self::Output {
        Vec2 { 
            x: self.x - other.x, 
            y: self.y - other.y 
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Self::Output {
        self * &scalar
    }
}

impl Mul<&f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: &f64) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟨{}, {}⟩", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2{
        Vec2 { x, y }
    }

    pub fn new_fill(n: f64) -> Vec2 {
        Vec2 { x: n, y: n }
    } 

    pub fn zero() -> Vec2 {
        Vec2::new_fill(0.)
    }

    /// rotates the vector by a given theta in radians
    pub fn rotate_rad(&self, theta: f64) -> Vec2 {
        // rotation matrix
        // [ cos(Θ)   -sin(Θ) ] [ vx ]
        // [ sin(Θ)   cos(Θ)  ] [ vy ] 
        //   a^       b^
        
        let (sin_theta, cos_theta) = theta.sin_cos();

        Vec2 { 
            x: self.x * cos_theta + self.y * (-sin_theta), 
            y: self.x * sin_theta + self.y * cos_theta
        }
    }

    pub fn swap_xy(&self) -> Vec2 {
        Vec2 { 
            x: self.y, 
            y: self.x 
        }
    }
}

impl VecMath for Vec2 { 
    fn distance(&self, other: &Self) -> f64 {
        f64::sqrt(
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
        )
    }

    fn length(&self) -> f64 {
        f64::sqrt(
            (self.x * self.x) + (self.y * self.y)
        )
    }

    fn normalize(&self) -> Self {
        let dist = self.length();
        Vec2 { 
            x: self.x / dist, 
            y: self.y / dist
        }
    }

    fn lerp(&self, other: &Self, t: f64) -> Self {
        self + &((other - self) * t)
    }
}