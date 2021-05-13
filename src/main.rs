use rrt::hit::HittableList;
use rrt::material::{Dielectric, Lambertian, Metal};
use rrt::ray::Ray;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;
use rrt::{camera::Camera, random::rand_uniform};

use std::sync::Arc;

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth >= 50 {
        return Vec3::zero();
    }
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(hr) => match hr.material.scatter(r, &hr) {
            Some((scattered, att)) => {
                return att * color(&scattered, world, depth + 1);
            }
            None => {
                return Vec3::zero();
            }
        },
        None => {
            let ud = r.direction().unit_vector();
            let t = 0.5 * (ud.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn r2() -> f32 {
    rand_uniform() * rand_uniform()
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;

            let center = Vec3::new(a + 0.9 * rand_uniform(), 0.2, b + 0.9 * rand_uniform());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = rand_uniform();
                if choose_mat < 0.8 {
                    let s = Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Vec3::new(r2(), r2(), r2()))),
                    ));
                    world.list.push(s);
                } else if choose_mat < 0.95 {
                    let s = Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rand_uniform()),
                                0.5 * (1.0 + rand_uniform()),
                                0.5 * (1.0 + rand_uniform()),
                            ),
                            0.0,
                        )),
                    ));
                    world.list.push(s);
                } else {
                    let s = Box::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))));
                    world.list.push(s);
                }
            }
        }
    }

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    let world = random_scene();
    dbg!(&world.list.len());

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        dbg!(j);
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (rand_uniform() + i as f32) / nx as f32;
                let v = (rand_uniform() + j as f32) / ny as f32;

                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

            println!("{} {} {}", col.r(), col.g(), col.b());
        }
    }
}
