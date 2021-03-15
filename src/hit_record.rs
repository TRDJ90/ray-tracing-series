use crate::ray::Ray;
use crate::vec3::{Point3D, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub position: Point3D,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(& mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face { self.normal = outward_normal.clone() } else { self.normal = -outward_normal.clone(); }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}