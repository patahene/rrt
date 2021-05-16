use clap::{App, Arg};
use once_cell::sync::Lazy;
use rrt::camera::Camera;
use rrt::hit::HittableList;
use rrt::material::{Dielectric, Lambertian, MaterialKind, Metal};
use rrt::model::ramiel;
use rrt::random::rand_uniform;
use rrt::renderer::rendering;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;

const NX: u32 = 1920;
const NY: u32 = 1080;
const NS: u32 = 100;

static SCENE: Lazy<HittableList> = Lazy::new(|| random_scene());
static CAM: Lazy<Camera> = Lazy::new(|| {
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.1;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect = NX as f32 / NY as f32;
    Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, focus_dist)
});

fn r2() -> f32 {
    rand_uniform() * rand_uniform()
}

fn random_scene() -> HittableList {
    fastrand::seed(0);
    let mut world = HittableList::new();

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        MaterialKind::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;

            let center = Vec3::new(a + 0.9 * rand_uniform(), 0.2, b + 0.9 * rand_uniform());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 && rand_uniform() < 0.7 {
                let choose_mat = rand_uniform();
                if choose_mat < 0.8 {
                    let s = Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialKind::Lambertian(Lambertian::new(Vec3::new(r2(), r2(), r2()))),
                    ));
                    world.list.push(s);
                } else if choose_mat < 0.95 {
                    let s = Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialKind::Metal(Metal::new(
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
                    let s = Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialKind::Dielectric(Dielectric::new(1.5, Vec3::one())),
                    ));
                    world.list.push(s);
                }
            }
        }
    }

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        MaterialKind::Dielectric(Dielectric::new(1.5, Vec3::one())),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        MaterialKind::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        MaterialKind::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    for t in ramiel(Vec3::new(3.0, 1.0, 2.0), 1.0) {
        world.list.push(t);
    }

    world
}

fn main() {
    let matches = App::new("rtt")
        .version("0.1.0")
        .arg(
            Arg::with_name("thread")
                .short("t")
                .long("thread")
                .value_name("THREAD_NUM")
                .takes_value(true),
        )
        .get_matches();

    let thread: usize = matches.value_of("thread").unwrap_or("0").parse().unwrap();
    let start = std::time::SystemTime::now();
    rendering(NX, NY, NS, &CAM, &SCENE, thread);
    println!("{:?}", start.elapsed().unwrap());
}
