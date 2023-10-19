use crate::math::{Tri3d, vec3::Vec3};

pub struct Cube {
    pub mesh: Vec<Tri3d>,
}

impl Cube {
    pub fn new() -> Self {
        //todo really messy change later

        let mut v = [
            Vec3::new(1.,  1.,  1.),
            Vec3::new(-1., 1.,  1.),
            Vec3::new(-1., -1., 1.),
            Vec3::new(1.,  -1., 1.),
            Vec3::new(1.,  1.,  -1.),
            Vec3::new(-1., 1.,  -1.),
            Vec3::new(-1., -1., -1.),
            Vec3::new(1.,  -1., -1.),
        ];

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

    pub fn scale(self, scalar: f64) -> Self {
        let mut mesh = vec![];

        for tri in self.mesh.iter() {
            mesh.push(
                [
                    tri[0] * scalar,
                    tri[1] * scalar,
                    tri[2] * scalar,
                ]
            )
        }

        Cube { mesh }
    }

    pub fn translate(self, translation: Vec3) -> Self {
        let mut mesh = vec![];

        for tri in self.mesh.iter() {
            mesh.push(
                [
                    tri[0] + translation,
                    tri[1] + translation,
                    tri[2] + translation,
                ]
            )
        }

        Cube { mesh }
    }
}