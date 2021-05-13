use rrt::camera::Camera;
use rrt::hit::HittableList;
use rrt::ray::Ray;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;

use rand::{distributions::Standard, Rng};

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    match world.hit(r, 0.0, std::f32::MAX) {
        Some(hr) => {
            return 0.5
                * Vec3::new(
                    hr.normal.x() + 1.0,
                    hr.normal.y() + 1.0,
                    hr.normal.z() + 1.0,
                );
        }
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

    let seed: [u8; 32] = [0; 32];
    let mut rng: rand::prelude::StdRng = rand::SeedableRng::from_seed(seed);

    let mut world = HittableList::new();
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = world;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let s: f32 = rng.sample(Standard);
                let u = (s + i as f32) / nx as f32;

                let s: f32 = rng.sample(Standard);
                let v = (s + j as f32) / ny as f32;

                let r = cam.get_ray(u, v);
                // let p = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }
            col /= ns as f32;

            println!("{} {} {}", col.r(), col.g(), col.b());
        }
    }
}
