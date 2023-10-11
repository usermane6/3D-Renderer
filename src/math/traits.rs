pub trait VecMath {
    /// returns the length of the vector
    fn length(&self) -> f64;

    /// returns the normalized vector, the vector where the len is 1
    fn normalize(&self) -> Self;

    /// returns the distance between the vetor and another 
    fn distance(&self, other: &Self) -> f64;

    /// linear interpolation of vectors
    fn lerp(&self, other: &Self, t: f64) -> Self;
}