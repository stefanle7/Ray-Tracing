use glam::Vec3;

use crate::material::Material;
use crate::shape::{Hit, Shape};
use crate::transform::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub transform: Transform, 
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            transform: Transform::new(center, Vec3::ZERO, Vec3::splat(radius)),
            material,
        }
    }
}

impl Shape for Sphere {
    // lecture 7 slide 17
    fn hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit> {
        let center = self.transform.translate;
        let radius = self.transform.scale.x;

        let ec = e - center;
        let a = d.dot(d); // A
        let b = 2.0 * d.dot(ec); // B
        let c = ec.dot(ec) - radius * radius; // C
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None; // the ray misses
        }
        let sq = disc.sqrt();
        let mut t = (-b - sq) / (2.0 * a); // near root first
        if t < t_min || t > t_max {
            t = (-b + sq) / (2.0 * a); // ...else the far root
            if t < t_min || t > t_max {
                return None;
            }
        }
        let p = e + t * d;
        Some(Hit {
            t,
            p,
            n: (p - center) / radius,
            material: self.material,
        })
    }
}