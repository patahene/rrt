use crate::hit::Hittable;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::MaterialKind;
use crate::triangle::Triangle;
use crate::vec3::Vec3;

pub fn ramiel(position: Vec3, scale: f32) -> Vec<Box<dyn Hittable + Send + Sync>> {
    let mut r: Vec<Box<dyn Hittable + Send + Sync>> = vec![];

    let mat = MaterialKind::Dielectric(Dielectric::new(
        1.1,
        Vec3::new(0.2, 0.2, 0.85),
    ));
    let hight_scale = 0.9;

    let t1 = Triangle::new(
        Vec3::new(1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, 1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, 1.0) * scale + position,
        mat,
    );
    let t2 = Triangle::new(
        Vec3::new(0.0, 1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(-1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, 1.0) * scale + position,
        mat,
    );
    let t3 = Triangle::new(
        Vec3::new(-1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, -1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, 1.0) * scale + position,
        mat,
    );
    let t4 = Triangle::new(
        Vec3::new(0.0, -1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, 1.0) * scale + position,
        mat,
    );
    let t5 = Triangle::new(
        Vec3::new(1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, 1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, -1.0) * scale + position,
        mat,
    );
    let t6 = Triangle::new(
        Vec3::new(0.0, 1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(-1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, -1.0) * scale + position,
        mat,
    );
    let t7 = Triangle::new(
        Vec3::new(-1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, -1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, -1.0) * scale + position,
        mat,
    );
    let t8 = Triangle::new(
        Vec3::new(0.0, -1.0 * hight_scale, 0.0) * scale + position,
        Vec3::new(1.0, 0.0, 0.0) * scale + position,
        Vec3::new(0.0, 0.0, -1.0) * scale + position,
        mat,
    );

    r.push(Box::new(t1));
    r.push(Box::new(t2));
    r.push(Box::new(t3));
    r.push(Box::new(t4));
    r.push(Box::new(t5));
    r.push(Box::new(t6));
    r.push(Box::new(t7));
    r.push(Box::new(t8));

    r
}

pub fn wall() -> Vec<Box<dyn Hittable + Send + Sync>> {
    let pc = MaterialKind::Lambertian(Lambertian::new(Vec3::new(1.0, 1.0, 1.0)));
    let ps: f32 = 10.0;

    let mut r: Vec<Box<dyn Hittable + Send + Sync>> = vec![];
    // ground plane
    r.push(Box::new(Triangle::new(
        Vec3::new(-ps, 0.0, -ps),
        Vec3::new(ps, 0.0, -ps),
        Vec3::new(-ps, 0.0, ps),
        pc,
    )));
    r.push(Box::new(Triangle::new(
        Vec3::new(ps, 0.0, ps),
        Vec3::new(ps, 0.0, -ps),
        Vec3::new(-ps, 0.0, ps),
        pc,
    )));

    // y-z
    r.push(Box::new(Triangle::new(
        Vec3::new(-ps, 0.0, -ps),
        Vec3::new(-ps, ps, -ps),
        Vec3::new(-ps, 0.0, ps),
        pc,
    )));
    r.push(Box::new(Triangle::new(
        Vec3::new(-ps, ps, ps),
        Vec3::new(-ps, ps, -ps),
        Vec3::new(-ps, 0.0, ps),
        pc,
    )));

    // y-x
    r.push(Box::new(Triangle::new(
        Vec3::new(-ps, 0.0, -ps),
        Vec3::new(-ps, ps, -ps),
        Vec3::new(ps, 0.0, -ps),
        pc,
    )));
    r.push(Box::new(Triangle::new(
        Vec3::new(ps, ps, -ps),
        Vec3::new(-ps, ps, -ps),
        Vec3::new(ps, 0.0, -ps),
        pc,
    )));

    return r;
}
