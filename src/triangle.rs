use crate::hit::{HitRecord, Hittable};
use crate::material::MaterialKind;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Triangle {
    points: [Vec3; 3],
    material: MaterialKind,
    normal_vector: Vec3,
    distance: f32,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, mat: MaterialKind) -> Triangle {
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac).unit_vector();
        let d = a * n;
        let d = -(d.x() + d.y() + d.z());

        // let n = Vec3::new(1.0, -1.0, 3.0);
        // let d = 1.0;
        // dbg!(n);
        // dbg!(d);
        Triangle {
            points: [a, b, c],
            material: mat,
            normal_vector: n,
            distance: d,
        }
    }

    fn update(&mut self) {
        let [a, b, c] = self.points;
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac).unit_vector();
        let d = a * n;
        let d = -(d.x() + d.y() + d.z());
        self.normal_vector = n;
        self.distance = d;
    }

    pub fn move_x(&mut self, x: f32) -> &mut Triangle {
        let v = Vec3::new(x, 0.0, 0.0);
        for i in 0..3 {
            self.points[i] += v;
        }
        self.update();
        self
    }

    pub fn move_y(&mut self, y: f32) -> &mut Triangle {
        let v = Vec3::new(0.0, y, 0.0);
        for i in 0..3 {
            self.points[i] += v;
        }
        self.update();
        self
    }

    pub fn move_z(&mut self, z: f32) -> &mut Triangle {
        let v = Vec3::new(0.0, 0.0, z);
        for i in 0..3 {
            self.points[i] += v;
        }
        self.update();
        self
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // http://yamatyuu.net/other/point1/index.html
        let d = self.normal_vector * r.origin();
        let d = d.x() + d.y() + d.z() + self.distance;
        let n = self.normal_vector * r.direction();
        let n = n.x() + n.y() + n.z();
        let t = -d / n;
        // dbg!(t);
        if t < t_min || t > t_max {
            return None;
        }

        let p = t * r.direction() + r.origin();
        // dbg!(p);

        let [a, b, c] = self.points;
        let ab = b - a;
        let bp = p - b;

        let bc = c - b;
        let cp = p - c;

        let ca = a - c;
        let ap = p - a;

        let c1 = ab.cross(bp);
        let c2 = bc.cross(cp);
        let c3 = ca.cross(ap);

        if c1.dot(c2) > 0.0 && c1.dot(c3) > 0.0 {
            // dbg!("hit");
            let mut rec = HitRecord::new(self.material);
            rec.t = t;
            rec.p = p;

            // dbg!(self.normal_vector.dot(r.direction().unit_vector()));
            rec.normal = if self.normal_vector.dot(r.direction().unit_vector()) < 0.0 {
                self.normal_vector
            } else {
                self.normal_vector * -1.0
            };
            Some(rec)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{material::Metal, triangle::*};
    #[test]
    fn test() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(2.0, 0.0, 0.0);
        let c = Vec3::new(0.0, 3.0, 0.0);

        let m = MaterialKind::Metal(Metal::new(Vec3::new(1.0, 1.0, 1.0), 0.1));
        let t = Triangle::new(a, b, c, m);
        // let o = Vec3::new(1.0, 1.0, 10.0);
        let o = Vec3::new(10.0, 1.0, 10.0);
        let d = Vec3::new(0.0, 0.0, -1.0).unit_vector();
        let r = Ray::new(o, d);
        let res = t.hit(&r, 0.0, std::f32::MAX);
        dbg!(res);
        // match res {
        //     Some(hr) => hr.p,
        //     None => {
        //         unreachable!();
        //     }
        // }
    }
}
