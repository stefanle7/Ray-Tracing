use glam::Vec3;

use crate::material::Material;
use crate::shape::{Hit, Shape};
use crate::transform::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub transform: Transform, 
    pub n: Vec3,              
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            transform: Transform::new(point, Vec3::ZERO, Vec3::ONE),
            n: normal.normalize(),
            material,
        }
    }
}

// lecture 7 slide 11
impl Shape for Plane {
    fn hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit> {
        let a = self.transform.translate; // a point on the plane

        let denom = d.dot(self.n);
        if denom.abs() < 1e-6 {
            return None; // parallel to the plane
        }

        let t = (a - e).dot(self.n) / denom; // the whole solution
        if t < t_min || t > t_max {
            return None; // behind us, or too far
        }

        let n = if denom < 0.0 { self.n } else { -self.n }; // face the ray
        Some(Hit {
            t,
            p: e + t * d,
            n,
            material: self.material,
        })
    }
}