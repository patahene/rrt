use rrt::vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2) * 255.99;
            println!("{} {} {}", col.r() as i32, col.g() as i32, col.b() as i32);
        }
    }
}
