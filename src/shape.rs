use glam::Vec3;

use crate::material::Material;

// What a hit test returns: everything the shader needs
// (t, p, n from lecture 7 slide 5)
#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub t: f32,             // ray parameter at the hit
    pub p: Vec3,            // hit point = e + t * d
    pub n: Vec3,            // unit surface normal at p
    pub material: Material, // added this myself
}

pub trait Shape {
    fn hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit>;
}