use image::ImageBuffer;
use once_cell::sync::Lazy;
use rrt::camera::Camera;
use rrt::hit::HittableList;
use rrt::material::{Dielectric, Lambertian, MaterialKind, Metal};
use rrt::random::rand_uniform;
use rrt::ray::Ray;
use rrt::sphere::Sphere;
use rrt::vec3::Vec3;

const NX: u32 = 1920 * 2;
const NY: u32 = 1080 * 2;
const NS: u32 = 100;

static CAM: Lazy<Camera> = Lazy::new(|| {
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        NX as f32 / NY as f32,
        aperture,
        dist_to_focus,
    )
});

static SCENE: Lazy<HittableList> = Lazy::new(|| random_scene());

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
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
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
    world
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        dbg!(j);
        let r = h.await.unwrap();
        for (i, col) in r.iter().enumerate() {
            ib.put_pixel(i as u32, j as u32, image::Rgb([col.r(), col.g(), col.b()]));
        }
    }
    ib.save("my_scene.png").unwrap();

    Ok(())
}
