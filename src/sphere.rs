use crate::hit_record::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3D, Vec3};

pub struct Sphere {
    center: Point3D,
    radius: f32,
}

impl Sphere {
    pub fn new(center: &Point3D, radius: f32) -> Sphere {
        Sphere {
            center: center.clone(),
            radius: radius,
        }
    } 
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = (half_b * half_b) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt(); 

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.position = ray.at(record.t);
        
        let outward_normal = &(&record.position - &self.center) / self.radius;
        record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}