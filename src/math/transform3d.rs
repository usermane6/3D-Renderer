use super::{vec4::Vec4, mat4::Mat4};

#[derive(Debug, Clone, Copy)]
pub struct Transform3d {
    pub rotation: f64,
    pub scale: f64,
    pub translation: Vec4,
}

impl Transform3d {
    pub fn new(rotation: f64, scale: f64, translation: Vec4) -> Self {
        Transform3d { rotation, scale, translation }
    }

    pub fn new_empty() -> Self {
        Transform3d { rotation: 0., scale: 1., translation: Vec4::zero() }
    }

    pub fn rotation_mat(&self) -> Mat4 {
        Mat4::rotation_rads(self.rotation)
    }

    pub fn scale_mat(&self) -> Mat4 {
        Mat4::scale(self.scale)
    }

    pub fn translation_mat(&self) -> Mat4 {
        Mat4::translation(self.translation)
    }
}