use once_cell::sync::Lazy;
use rrt::camera::Camera;
use rrt::hit::HittableList;
use rrt::material::{Lambertian, MaterialKind, Metal};
use rrt::model::ramiel;
use rrt::model::wall;
use rrt::renderer::rendering;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;

const NX: u32 = 1920 / 2;
const NY: u32 = 1080 / 2;
const NS: u32 = 100;

static SCENE: Lazy<HittableList> = Lazy::new(|| test_scene());
static CAM: Lazy<Camera> = Lazy::new(|| {
    let lookfrom = Vec3::new(10.0, 20.0, 50.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.1;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect = NX as f32 / NY as f32;

    Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, focus_dist)
});

#[allow(dead_code)]
fn test_scene() -> HittableList {
    let mut world = HittableList::new();

    let metal = MaterialKind::Metal(Metal::new(Vec3::new(0.9, 0.9, 0.9), 0.0));
    let lam_g = MaterialKind::Lambertian(Lambertian::new(Vec3::new(0.2, 0.8, 0.2)));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        0.1,
        MaterialKind::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(5.0, 0.0, 0.0),
        0.1,
        MaterialKind::Lambertian(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))),
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 5.0, 0.0),
        0.1,
        MaterialKind::Lambertian(Lambertian::new(Vec3::new(0.0, 1.0, 0.0))),
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 5.0),
        0.1,
        MaterialKind::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))),
    )));

    for t in ramiel(Vec3::new(0.0, 2.0, 0.0), 2.0) {
        world.list.push(t);
    }

    for t in wall() {
        world.list.push(t);
    }

    for i in 0..20 {
        let mat = if i % 2 == 0 { metal } else { lam_g };
        let i = i as f32;
        let d = std::f32::consts::PI * 2.0 / 20.0;
        let x = (d * i).sin() * 5.0;
        let z = (d * i).cos() * 2.0;

        world
            .list
            .push(Box::new(Sphere::new(Vec3::new(x, 0.5, z), 0.5, mat)));
    }

    for i in 0..20 {
        let mat = if i % 2 == 1 { metal } else { lam_g };
        let i = i as f32;
        let d = std::f32::consts::PI * 2.0 / 20.0;
        let x = (d * i).sin() * 5.0;
        let z = (d * i).cos() * 5.0;

        world
            .list
            .push(Box::new(Sphere::new(Vec3::new(x, 5.0, z), 0.5, mat)));
    }

    world
}

fn main() {
    let start = std::time::SystemTime::now();
    rendering(NX, NY, NS, &CAM, &SCENE, 0);
    println!("{:?}", start.elapsed().unwrap());
}
