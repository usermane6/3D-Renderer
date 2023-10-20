use std::ops::Index;

use super::{vec4::Vec4, color::Color, mat4::Mat4};

/// each point on the triangle is a 
#[derive(Debug, Clone, Copy)]
pub struct Tri3d {
    points: [Vec4; 3],
    color: Color
}

impl Tri3d {
    pub fn apply_transform(self, t: Mat4) -> Tri3d {
        let mut points = [Vec4::zero(); 3];

        for (i, p) in self.points.iter().enumerate() {
            points[i] = t * p;
        }

        Tri3d { points, color: self.color }
    }
}

impl Index<usize> for Tri3d {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}