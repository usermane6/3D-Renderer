use crate::math::{tri3d::Tri3d, transform3d::{Transform3d, self}, mat4::Mat4, vec4::Vec4, color::{Color, Colors}};

#[derive(Debug, Clone)]
pub struct Object3d {
    pub key: u32,
    pub transform: Transform3d,
    pub mesh: Vec<Tri3d>,
}

impl Object3d {
    pub fn cube() -> Self {
        let vs: [Vec4; 8] = [
            Vec4::new( 1.,  1.,  1.,  1.),
            Vec4::new(-1.,  1.,  1.,  1.),
            Vec4::new(-1., -1.,  1.,  1.),
            Vec4::new( 1., -1.,  1.,  1.),
            Vec4::new( 1.,  1., -1.,  1.),
            Vec4::new(-1.,  1., -1.,  1.),
            Vec4::new(-1., -1., -1.,  1.),
            Vec4::new( 1., -1., -1.,  1.),
        ];

        let mesh: Vec<Tri3d> = vec![
            Tri3d::new([vs[0], vs[1], vs[2]], Color::new_color(Colors::RED)), 
            Tri3d::new([vs[0], vs[2], vs[3]], Color::new_color(Colors::RED)),
            Tri3d::new([vs[4], vs[0], vs[3]], Color::new_color(Colors::GREEN)),
            Tri3d::new([vs[4], vs[3], vs[7]], Color::new_color(Colors::GREEN)),
            Tri3d::new([vs[5], vs[4], vs[7]], Color::new_color(Colors::BLUE)),
            Tri3d::new([vs[5], vs[7], vs[6]], Color::new_color(Colors::BLUE)),
            Tri3d::new([vs[1], vs[5], vs[6]], Color::new_color(Colors::YELLOW)),
            Tri3d::new([vs[1], vs[6], vs[2]], Color::new_color(Colors::YELLOW)),
            Tri3d::new([vs[4], vs[5], vs[1]], Color::new_color(Colors::MAGENTA)),
            Tri3d::new([vs[4], vs[1], vs[0]], Color::new_color(Colors::MAGENTA)),
            Tri3d::new([vs[2], vs[6], vs[7]], Color::new_color(Colors::CYAN)),
            Tri3d::new([vs[2], vs[7], vs[3]], Color::new_color(Colors::CYAN)),
        ];

        Object3d { key: 0, transform: Transform3d::new_empty(), mesh }
    }
}