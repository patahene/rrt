use rrt::hit::HittableList;
use rrt::ray::Ray;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;

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

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    let mut world = HittableList::new();
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r, &world) * 255.99;
            println!("{} {} {}", col.r() as i32, col.g() as i32, col.b() as i32);
        }
    }
}
