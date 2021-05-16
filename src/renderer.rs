use crate::camera::Camera;
use crate::hit::HittableList;
use crate::material::MaterialKind;
use crate::random::rand_uniform;
use crate::ray::Ray;
use crate::vec3::Vec3;
use image::ImageBuffer;

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

pub fn rendering(
    width: u32,
    height: u32,
    sampling_num: u32,
    cam: &'static Camera,
    scnene: &'static HittableList,
    thread_num: usize,
) {
    let mut runtime = tokio::runtime::Builder::new_multi_thread();
    if thread_num > 0 {
        runtime.worker_threads(thread_num);
    }
    runtime.enable_all().build().unwrap().block_on(async {
        let mut jh = vec![];
        {
            for j in 0..height {
                let mut row = vec![];
                let h = tokio::spawn(async move {
                    for i in 0..width {
                        let mut col = Vec3::zero();
                        for _ in 0..sampling_num {
                            let u = (rand_uniform() + i as f32) / width as f32;
                            let v = (rand_uniform() + (height - j - 1) as f32) / height as f32;
                            let r = cam.get_ray(u, v);
                            col += color(&r, scnene, 0);
                        }
                        col /= sampling_num as f32;
                        col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
                        row.push(col);
                    }
                    row
                });
                jh.push(h);
            }
        }

        let mut ib = ImageBuffer::new(width, height);
        for (j, h) in jh.iter_mut().enumerate() {
            let r = h.await.unwrap();
            for (i, col) in r.iter().enumerate() {
                ib.put_pixel(i as u32, j as u32, image::Rgb([col.r(), col.g(), col.b()]));
            }
        }
        ib.save("my_scene.png").unwrap();
    });
}
