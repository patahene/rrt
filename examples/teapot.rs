use once_cell::sync::Lazy;
use rrt::camera::Camera;
use rrt::hit::HittableList;
use rrt::model::load_obj;
use rrt::renderer::rendering;
use rrt::vec3::Vec3;

const NX: u32 = 1920 / 2;
const NY: u32 = 1080 / 2;
const NS: u32 = 100;

static SCENE: Lazy<HittableList> = Lazy::new(|| test_scene());
static CAM: Lazy<Camera> = Lazy::new(|| {
    let lookfrom = Vec3::new(10.0, 5.0, 10.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.1;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect = NX as f32 / NY as f32;

    Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, focus_dist)
});

fn test_scene() -> HittableList {
    let mut world = HittableList::new();
    let teapot = load_obj("model/teapot.obj").unwrap();

    for t in teapot {
        world.list.push(t);
    }
    dbg!(world.list.len());
    world
}

fn main() {
    let start = std::time::SystemTime::now();
    rendering(NX, NY, NS, &CAM, &SCENE, 6, "teapot.png", false);
    println!("{:?}", start.elapsed().unwrap());
}
