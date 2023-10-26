use super::{vec4::Vec4, mat4::Mat4, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Transform3d {
    pub rotation: Vec3,
    pub scale: f64,
    pub translation: Vec4,
}

impl Transform3d {
    pub fn new(rotation: Vec3, scale: f64, translation: Vec4) -> Self {
        Transform3d { rotation, scale, translation }
    }

    pub fn new_empty() -> Self {
        Transform3d { rotation: Vec3::zero(), scale: 1., translation: Vec4::zero() }
    }

    pub fn rotation_x_mat(&self) -> Mat4 {
        Mat4::rotation_x_rads(self.rotation.x)
    }

    pub fn rotation_y_mat(&self) -> Mat4 {
        Mat4::rotation_y_rads(self.rotation.y)
    }

    pub fn rotation_z_mat(&self) -> Mat4 {
        Mat4::rotation_z_rads(self.rotation.z)
    }

    pub fn rotation_mat(&self) -> Mat4 {
        self.rotation_x_mat() * self.rotation_y_mat() * self.rotation_z_mat()
    }

    pub fn scale_mat(&self) -> Mat4 {
        Mat4::scale(self.scale)
    }

    pub fn translation_mat(&self) -> Mat4 {
        Mat4::translation(self.translation)
    }
}