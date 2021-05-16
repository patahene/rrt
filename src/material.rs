use crate::hit::HitRecord;
use crate::random::rand_uniform;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum MaterialKind {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand_uniform(), rand_uniform(), rand_uniform())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Lambertian {
        Lambertian { albedo: a }
    }
    // }

    // impl Material for Lambertian {
    pub fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        Some((scattered, self.albedo))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: Vec3, f: f32) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
    // }

    // impl Material for Metal {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        // let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}


#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    ref_idx: f32,
    albedo: Vec3,
}

impl Dielectric {
    pub fn new(ri: f32, a: Vec3) -> Dielectric {
        Dielectric {
            ref_idx: ri,
            albedo: a,
        }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let attenuation = self.albedo;
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(rec.normal) > 0.0 {
            (
                -rec.normal,
                self.ref_idx,
                self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length(),
            )
        } else {
            (
                rec.normal,
                1.0 / self.ref_idx,
                -r_in.direction().dot(rec.normal) / r_in.direction().length(),
            )
        };

        let scattered = match refract(r_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rand_uniform() < schlick(cosine, self.ref_idx) {
                    Ray::new(rec.p, reflected)
                } else {
                    Ray::new(rec.p, refracted)
                }
            }
            None => Ray::new(rec.p, reflected),
        };

        Some((scattered, attenuation))
    }
}
