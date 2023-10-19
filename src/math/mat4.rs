use::std::ops::{Add, Sub, Mul};
use super::vec4::Vec4;
use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub vals: [f64; 16],
}

impl Mul<&Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, a: &Vec4) -> Self::Output {
        Vec4 {
            x: self.vals[0]  * a.x + self.vals[1]  * a.y + self.vals[2]  * a.z + self.vals[3]  * a.w,
            y: self.vals[4]  * a.x + self.vals[5]  * a.y + self.vals[6]  * a.z + self.vals[7]  * a.w,
            z: self.vals[8]  * a.x + self.vals[9]  * a.y + self.vals[10] * a.z + self.vals[11] * a.w,
            w: self.vals[12] * a.x + self.vals[13] * a.y + self.vals[14] * a.z + self.vals[15] * a.w,
        }
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    /// [a][b] = [c]
    fn mul(self, other: Self) -> Self::Output {
        let mut vals = [0.; 16];

        for i in 0usize..4usize {
            for j in 0usize..4usize {
                
                let id = (4 * i) + j;
            
                for k in 0..4 {
                    vals[id] += self.vals[(i * 4) + k] * other.vals[j + (k * 4)];
                }
            }
        }

        Mat4::new(vals)
    }
}

impl Mat4 {
    pub fn new(vals: [f64; 16]) -> Self {
        Mat4 { vals }
    }  

    /// translation by a given vector
    pub fn translaion(a: Vec4) -> Self {
        Mat4 {
            vals: [
                1.,  0.,  0.,  a.x,
                0.,  1.,  0.,  a.y,
                0.,  0.,  1.,  a.z,
                0.,  0.,  0.,  1.,
            ]
        }
    }

    /// produces a scaling transformation matrix, where s is a scalar
    pub fn scale(s: f64) -> Self {
        Mat4 {
            vals: [
                s,   0.,  0.,  0.,
                0.,  s,   0.,  0.,
                0.,  0.,  s,   0.,
                0.,  0.,  0.,  1.,
            ]
        }
    }

    /// rotation around the y axis
    pub fn rotation_rads(theta: f64) -> Self {
        let (sin, cos) = theta.sin_cos();
        
        Mat4 {
            vals: [
                cos,  0., -sin,  0.,
                 0.,  1.,   0.,  0.,
                sin,  0.,  cos,  0.,
                 0.,  0.,   0.,  1.,
            ]
        }
    }
}