use std::fmt;
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

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, a: Vec4) -> Self::Output {
        self * &a
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    /// [a][b] = [c]
    fn mul(self, other: Self) -> Self::Output {
        let mut vals = [0.; 16];

        for row in 0usize..4usize {      // row
            for col in 0usize..4usize {  // col
                
                let id = (4 * row) + col;
            
                for k in 0..4 {
                    // iterate through the rows of [a] and the columns of [b]
                    vals[id] += self.vals[(row * 4) + k] * other.vals[col + (k * 4)];
                }
            }
        }

        Mat4::new(vals)
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         // formatting of matrix into strings so they can be printed to the console

        let mut final_string = String::new();
        for y in 0..4 {
            for x in 0..4 {
                let mut val = self.get(x, y).to_string();
                let mut gap = "      ".to_string();

                if val.len() > gap.len() - 1 { // minus 1 so that there is a space between numbers, even at max length
                    val.replace_range(5..val.len(), ""); // truncate numbers, so the don't upset alignments
                }


                gap.replace_range(0..val.len(), ""); // adjust gap size between #s so #s are aligned
                
                final_string.push_str(gap.as_str());
                final_string.push_str(val.as_str());

            }

            final_string.push_str("\n")
        }

        write!(f, "{}", final_string)
    }
}

impl Mat4 {
    pub fn new(vals: [f64; 16]) -> Self {
        Mat4 { vals }
    }  

    /// translation by a given vector
    pub fn translation(a: Vec4) -> Self {
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

    /// d is the distance from the 
    pub fn projection(d: f64) -> Self {
        Mat4 { 
            vals: [
                d,   0.,  0.,  0., 
                0.,  d,   0.,  0., 
                0.,  0.,  1.,  0., 
                0.,  0.,  0.,  1., 
            ]
        }
    }

    /// takes the projection and places it on the canvas, returns 2d homogeneous coords, with a zero in the w position
    ///- v_w and v_h are viewport width / height
    ///- s_w and s_h are screen width / height
    pub fn onto_2d(v_w: f64, v_h: f64, s_w: f64, s_h: f64) -> Self {
        Mat4 { 
            vals: [
                s_w / v_w,   0.,          0.,          0., 
                0.,          s_h / v_h,   0.,          0., 
                0.,          0.,          1.,          0., 
                0.,          0.,          0.,          1., 
            ]
        }
    }

    /// Previous transformations will have the "center" of the projection be one of the corners
    /// this adjusts points to fix that
    pub fn fix_to_center(s_w: f64, s_h: f64) -> Self {
        Mat4 { 
            vals: [
                1.,  0.,  0.,  s_w / 2., 
                0.,  1.,  0.,  s_h / 2., 
                0.,  0.,  1.,  0., 
                0.,  0.,  0.,  1., 
            ]
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        let id = 4 * y + x;
        self.vals[id]
    }
}