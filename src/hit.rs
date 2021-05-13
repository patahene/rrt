use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: vec![] }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            match h.as_ref().hit(r, t_min, closest_so_far) {
                Some(hr) => {
                    hit_anything = true;
                    closest_so_far = hr.t;
                    rec = hr;
                }
                None => {}
            }
        }
        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}
