use glam::{Mat4, Vec3}; // using glam for vector and matrix math
use std::fmt; // using std


// same exact file from assignment 1
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    // constructor
    pub fn new(origin: Vec3, direction: Vec3) -> Self { // creating a new ray with origin and direction
        Self {origin, direction}
    }

    // Accessor methods for origin and direction
    pub fn origin(&self) -> Vec3 { // return a position vector
        self.origin
    }
    pub fn direction(&self) -> Vec3 { // returns a direction vector
        self.direction
    }

    // Method to evaluate a point of the ray given floating point value t
    pub fn evaluate (&self, t: f32) -> Vec3 { //returns a position in vector form
        self.origin + t * self.direction // returns a point along the ray at distance t from the origin
    }

    // Method to transform the ray by a transformation matrix
    pub fn transform(&self, matrix: Mat4) -> Self { // returns a ray
        let new_origin = matrix.transform_point3(self.origin); // transforming the origin point
        let new_direction = matrix.transform_vector3(self.direction); // transforming the direction vector
        Self::new(new_origin, new_direction) // returning a new ray with transformed origin and direction
    }
}

impl fmt::Display for Ray { // display for the ray

    // writing text into a buffer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Hello!") testing
        write!(f, "Ray {{ origin: {}, direction: {} }}", self.origin, self.direction)
    }
}

// test methods
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // my test to create a ray with origin and direction and see if they work
    fn new_and_accessors() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin(), origin);
        assert_eq!(r.direction(), direction);
    }

    #[test] // my test to check whether multiplying by t=0 returns to the origin
    fn evaluate_zero_returns_origin() {
        let r = Ray::new(Vec3::new(5.0, -1.0, 2.0), Vec3::X);
        assert_eq!(r.evaluate(0.0), r.origin());
    }

    #[test] // my test to check that t scales the direction vector
    fn evaluate_scales_unnormalized_direction() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 3.0, 0.0));
        assert_eq!(r.evaluate(2.0), Vec3::new(1.0, 8.0, 3.0));
    }

    #[test] // my test to check translation moves the origin but leaves the direction unchanged
    fn transform_translation_moves_origin_only() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::Z);
        let m = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let t = r.transform(m);
        assert_eq!(t.origin(), Vec3::new(11.0, 2.0, 3.0));
        assert_eq!(t.direction(), Vec3::Z);
    }

    #[test] // my test to check whether the display format of the ray is correct
    fn display_formats_origin_and_direction() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        assert_eq!(
            format!("{}", r),
            "Ray { origin: [0, 0, 0], direction: [1, 0, 0] }"
        );
    }
}
