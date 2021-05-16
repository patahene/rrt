use clap::{App, Arg};
use image::ImageBuffer;
use once_cell::sync::Lazy;
use rrt::camera::Camera;
use rrt::hit::HittableList;
use rrt::material::{Dielectric, Lambertian, MaterialKind, Metal};
use rrt::model::ramiel;
use rrt::model::wall;
use rrt::random::rand_uniform;
use rrt::ray::Ray;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;

const NX: u32 = 1920;
const NY: u32 = 1080;
const NS: u32 = 100;

// const NX: u32 = 1920 / 5;
// const NY: u32 = 1080 / 5;
// const NS: u32 = 100;

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

// static SCENE: Lazy<HittableList> = Lazy::new(|| random_scene());
// static CAM: Lazy<Camera> = Lazy::new(|| {
//     let lookfrom = Vec3::new(13.0, 2.0, 3.0);
//     // let lookfrom = Vec3::new(5.0, 5.0, 5.0);
//     let lookat = Vec3::new(0.0, 0.0, 0.0);
//     let focus_dist = (lookfrom - lookat).length();
//     let aperture = 0.1;
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let vfov = 20.0;
//     let aspect = NX as f32 / NY as f32;
//     Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, focus_dist)
// });

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

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth >= 50 {
        return Vec3::zero();
    }
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(hr) => {
            let scatter_result = match hr.material {
                MaterialKind::Lambertian(m) => m.scatter(r, &hr),
                MaterialKind::Dielectric(m) => m.scatter(r, &hr),
                MaterialKind::Metal(m) => m.scatter(r, &hr),
                MaterialKind::DielectricWithAlbedo(m) => m.scatter(r, &hr),
            };

            match scatter_result {
                Some((scattered, att)) => {
                    return att * color(&scattered, world, depth + 1);
                }
                None => {
                    return Vec3::zero();
                }
            }
        }
        None => {
            let ud = r.direction().unit_vector();
            let t = 0.5 * (ud.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

#[allow(dead_code)]
fn r2() -> f32 {
    rand_uniform() * rand_uniform()
}

#[allow(dead_code)]
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
                        MaterialKind::Dielectric(Dielectric::new(1.5)),
                    ));
                    world.list.push(s);
                }
            }
        }
    }

    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        MaterialKind::Dielectric(Dielectric::new(1.5)),
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

async fn rendering() {
    let mut jh = vec![];
    {
        for j in 0..NY {
            let mut row = vec![];
            let h = tokio::spawn(async move {
                for i in 0..NX {
                    let mut col = Vec3::zero();
                    for _ in 0..NS {
                        let u = (rand_uniform() + i as f32) / NX as f32;
                        let v = (rand_uniform() + (NY - j - 1) as f32) / NY as f32;
                        let r = CAM.get_ray(u, v);
                        col += color(&r, &SCENE, 0);
                    }
                    col /= NS as f32;
                    col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
                    row.push(col);
                }
                row
            });
            jh.push(h);
        }
    }

    let mut ib = ImageBuffer::new(NX, NY);
    for (j, h) in jh.iter_mut().enumerate() {
        let r = h.await.unwrap();
        for (i, col) in r.iter().enumerate() {
            ib.put_pixel(i as u32, j as u32, image::Rgb([col.r(), col.g(), col.b()]));
        }
    }
    ib.save("my_scene.png").unwrap();
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
    let mut runtime = tokio::runtime::Builder::new_multi_thread();
    if thread > 0 {
        runtime.worker_threads(thread);
    }
    runtime.enable_all().build().unwrap().block_on(async {
        let start = std::time::SystemTime::now();
        rendering().await;
        println!("{:?}", start.elapsed().unwrap());
    });
}
