use crate::vec3::{Point3D, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3, 
}

impl Ray {
    pub fn new(origin: &Point3D, direction: &Vec3) -> Ray {
        Ray { 
            origin: origin.clone(), 
            direction: direction.clone(), 
        }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Point3D {
        &self.origin + &(&self.direction * t)
    }
}