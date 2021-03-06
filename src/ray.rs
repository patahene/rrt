use crate::vec3::Vec3;
#[allow(non_snake_case)]
pub struct Ray {
    A: Vec3, // ray origin
    B: Vec3, // ray direction
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { A: a, B: b }
    }

    pub fn origin(&self) -> Vec3 {
        self.A
    }

    pub fn direction(&self) -> Vec3 {
        self.B
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.A + t * self.B
    }
}
