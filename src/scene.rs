// iterate through a scene and hit test each shape

use glam::Vec3;

use crate::film::Color;
use crate::ray::Ray;
use crate::shape::{Hit, Shape};

// a point light: where it is, and its brightness/color (I in lecture 8)
pub struct Light {
    pub position: Vec3,
    pub intensity: Color,
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
    pub ambient: Color, // Ia: small "light from everywhere" (lecture 8 slide 13)
    pub background: Color,
}

// creates a new scene with no shapes or lights yet
impl Scene {
    pub fn new(background: Color, ambient: Color) -> Self {
        Self {
            shapes: Vec::new(),
            lights: Vec::new(),
            ambient,
            background,
        }
    }

    // adding a shape, either sphere or a plane wrapped in a box
    // Box<dyn Shape> is how Rust keeps different types in a list
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    // adding a point light
    pub fn add_light(&mut self, position: Vec3, intensity: Color) {
        self.lights.push(Light { position, intensity });
    }

    // Test the ray against every shape and keep the closest hit (lecture 7 slide 6).
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for shape in &self.shapes {
            if let Some(hit) = shape.hit(ray.origin(), ray.direction(), t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
        // return closest_hit
        closest_hit
    }

    // implementing shadows to the image
    pub fn shadow_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for shape in &self.shapes {
            if let Some(hit) = shape.hit(ray.origin(), ray.direction(), t_min, t_max) {
                return true;
            }
        }
        false
    }
}