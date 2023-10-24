use crate::math::{tri3d::Tri3d, transform3d::Transform3d, mat4::Mat4};

pub struct Object3d {
    pub key: u32,
    pub transform: Transform3d,
    pub mesh: Vec<Tri3d>,
}
