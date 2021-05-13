use rrt::hit::HittableList;
use rrt::material::{Lambertian, Metal};
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

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let cam = Camera::new();

    let mut world = HittableList::new();
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)),
    )));
    let world = world;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (rand_uniform() + i as f32) / nx as f32;
                let v = (rand_uniform() + j as f32) / ny as f32;

                let r = cam.get_ray(u, v);
                // let p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

            println!("{} {} {}", col.r(), col.g(), col.b());
        }
    }
}
