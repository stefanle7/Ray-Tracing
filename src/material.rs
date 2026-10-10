use crate::film::Color;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shape::Hit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub diffuse: Color, // diffuse color: the surface color
    pub specular: Color, // specular color: the highlight (0 = Lambert, no highlight)
    pub p: f32,     // shininess (Phong exponent)
}

// creating a new material 
impl Material {
    pub fn new(diffuse: Color, specular: Color, p: f32) -> Self {
        Self { diffuse, specular, p }
    }
}

// shading pixel, lecture 8 slide 23 will help me solve this
pub fn shade_pixel(hit: &Hit, ray: &Ray, scene: &Scene) -> Color {
    // step 1: find nearest hit (no hit = background color)
    // done by scene.hit() in render.rs before this function is called

    // step 2: get p, n, and nearest material
    let p = hit.p;
    let n = hit.n;
    let m = hit.material;

    // step 3: compute v, the unit vector from p toward the eye (new -- once per hit)
    let v = -ray.direction().normalize(); // slide 6 says v = -normalized(d)

    // step 4: ambient term L = ka * Ia
    let mut color = m.diffuse * scene.ambient;

    // step 5: for each light
    for light in &scene.lights {
        let l = (light.position - p).normalize(); // compute l, unit vector from p toward light
        let h = (v + l).normalize(); // compute h = normalize(v + l)
        color += m.diffuse * light.intensity * n.dot(l).max(0.0); // add diffuse term
        color += m.specular * light.intensity * n.dot(h).max(0.0).powf(m.p); // add specular term
    }

    // step 6: clamp L to [0, 1]
    color.clamp(Color::ZERO, Color::ONE)
}