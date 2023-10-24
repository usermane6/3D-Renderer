use std::ops::Index;
use super::{vec2::Vec2, color::Color};

#[derive(Debug, Clone, Copy)]
pub struct Tri2d {
    points: [Vec2; 3],
    pub color: Color
}

impl Tri2d {
    pub fn new(points: [Vec2; 3], color: Color) -> Self {
        Tri2d { points, color }
    }
}

impl Index<usize> for Tri2d {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}