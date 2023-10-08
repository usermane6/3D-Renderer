use crate::math::{Tri3d, vec3::Vec3};

pub struct Cube {
    pub mesh: Vec<Tri3d>,
}

impl Cube {
    pub fn new() -> Self {
        //todo really messy change later

        let s = 1.5;
        let o = Vec3::new(0., 2., 10.);

        let mut v = [
            Vec3::new(1., 1., 1.),
            Vec3::new(-1., 1., 1.),
            Vec3::new(-1., -1., 1.),
            Vec3::new(1., -1., 1.),
            Vec3::new(1., 1., -1.),
            Vec3::new(-1., 1., -1.),
            Vec3::new(-1., -1., -1.),
            Vec3::new(1., -1., -1.),
        ];

        for vert in 0..v.len() {
            v[vert] = (v[vert] * s) + o;
        }

        Cube { 
            mesh: vec![
                [v[0], v[1], v[2]],
                [v[0], v[2], v[3]],
                [v[4], v[0], v[3]],
                [v[4], v[3], v[7]],
                [v[5], v[4], v[7]],
                [v[5], v[7], v[6]],
                [v[1], v[5], v[6]],
                [v[1], v[6], v[2]],
                [v[4], v[5], v[1]],
                [v[4], v[1], v[0]],
                [v[2], v[6], v[7]],
                [v[2], v[7], v[3]],
            ]
        }
    }
}