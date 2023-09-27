use std::fmt;
use::std::ops::{Add, Sub, Mul};

pub trait VecMath {
    /// returns the length of the vector
    fn length(&self) -> f64;

    /// returns the normalized vector, the vector where the len is 1
    fn normalize(&self) -> Self;

    /// returns the distance between the vetor and another 
    fn distance(&self, other: Self) -> f64;
}

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Self) -> Self::Output {
        Vec2 { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        Vec2 { 
            x: self.x - other.x, 
            y: self.y - other.y 
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
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

    pub fn ZERO() -> Vec2 {
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
}

impl VecMath for Vec2 { 
    fn length(&self) -> f64 {
        f64::sqrt((self.x * self.x) + (self.y * self.y))
    }

    fn normalize(&self) -> Self {
        let dist = self.length();
        Vec2 { 
            x: self.x / dist, 
            y: self.y / dist
        }
    }

    fn distance(&self, other: Self) -> f64 {
        f64::sqrt(
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
        )
    }
}


pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
