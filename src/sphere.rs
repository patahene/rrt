use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            material: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminat = b * b - a * c;
        if discriminat > 0.0 {
            let mut rec = HitRecord::new(self.material.clone());
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
        }
        None
    }
}
