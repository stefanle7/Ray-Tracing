use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Transform {
    pub translate: Vec3, // where the shape sits in the world
    pub rotate: Vec3,    // degrees around the x, y and z axes
    pub scale: Vec3,     // size along each axis
}

impl Transform {
    pub fn new(translate: Vec3, rotate: Vec3, scale: Vec3) -> Self {
        Self {
            translate,
            rotate,
            scale,
        }
    }
}