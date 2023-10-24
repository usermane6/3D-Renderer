use std::ops::Index;
use super::{vec2::Vec2, vec4::Vec4, color::Color, mat4::Mat4, tri2d::Tri2d};

/// each point on the triangle is a homogeneous coordinate 
#[derive(Debug, Clone, Copy)]
pub struct Tri3d {
    points: [Vec4; 3],
    color: Color
}

impl Tri3d {
    pub fn new(points: [Vec4; 3], color: Color) -> Self {
        Tri3d { points, color }
    }

    pub fn apply_transform(self, t: Mat4) -> Tri3d {
        let mut points = [Vec4::zero(); 3];

        for (i, p) in self.points.iter().enumerate() {
            points[i] = t * p;
        }

        Tri3d { points, color: self.color }
    }
}

impl Into<Tri2d> for Tri3d {
    fn into(self) -> Tri2d {
        let projected_points: [Vec2; 3] = [
            self[0].into(),
            self[1].into(),
            self[2].into()
        ];
        Tri2d::new(projected_points, self.color)
    }
}

impl Index<usize> for Tri3d {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}