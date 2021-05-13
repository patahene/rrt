use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Sphere {
        Sphere {
            center: c,
            radius: r,
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
            let mut rec = HitRecord::new();
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
