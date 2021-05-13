use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> i32 {
        (self.e[0] as f32 * 255.99) as i32
    }
    pub fn g(&self) -> i32 {
        (self.e[1] as f32 * 255.99) as i32
    }
    pub fn b(&self) -> i32 {
        (self.e[2] as f32 * 255.99) as i32
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            -self.e[0] * other.e[2] - self.e[2] * other.e[0],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        let mut v = Vec3::zero();
        for i in 0..3 {
            v.e[i] = self.e[i] + other.e[i];
        }
        v
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        for i in 0..3 {
            self.e[i] += other.e[i];
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        let mut v = Vec3::zero();
        for i in 0..3 {
            assert_ne!(other.e[i], 0.0);
            v.e[i] = self.e[i] / other.e[i];
        }
        v
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        assert_ne!(other, 0.0);
        let mut v = Vec3::zero();
        for i in 0..3 {
            v.e[i] = self.e[i] / other;
        }
        v
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        for i in 0..3 {
            assert_ne!(other.e[i], 0.0);
            self.e[i] /= other.e[i];
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        assert_ne!(other, 0.0);
        for i in 0..3 {
            self.e[i] /= other;
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        let mut v = Vec3::zero();
        for i in 0..3 {
            v.e[i] = self.e[i] * other.e[i];
        }
        v
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        let mut v = Vec3::zero();
        for i in 0..3 {
            v.e[i] = self.e[i] * other;
        }
        v
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        let mut v = Vec3::zero();
        for i in 0..3 {
            v.e[i] = other.e[i] * self;
        }
        v
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self.e[i] *= rhs.e[i];
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..3 {
            self.e[i] *= rhs;
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        let mut v = Vec3::zero();
        for i in 0..3 {
            v.e[i] = self.e[i] - other.e[i];
        }
        v
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self.e[i] -= rhs.e[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::*;
    use float_eq::assert_float_eq;
    const ABS_DIFF_LIMIT: f32 = 0.0000002;

    #[test]
    fn length() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        assert_float_eq!(a.length(), 1.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 1.0, 0.0);
        assert_float_eq!(a.length(), 2.0f32.sqrt(), abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 1.0, 1.0);
        assert_float_eq!(a.length(), 3.0f32.sqrt(), abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_float_eq!(a.length(), 14.0f32.sqrt(), abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn squared_length() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        assert_float_eq!(a.squared_length(), 1.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 1.0, 0.0);
        assert_float_eq!(a.squared_length(), 2.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 1.0, 1.0);
        assert_float_eq!(a.squared_length(), 3.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_float_eq!(a.squared_length(), 14.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert_float_eq!(a.dot(b), 0.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 0.0);
        assert_float_eq!(a.dot(b), 1.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 1.0, 1.0);
        assert_float_eq!(a.dot(b), 8.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn unit_vector() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let u = a.unit_vector();
        assert_float_eq!(u.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(u.length(), 1.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let u = a.unit_vector();
        assert_float_eq!(u.x(), 1.0 / a.length(), abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(u.y(), 2.0 / a.length(), abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(u.z(), 3.0 / a.length(), abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(u.length(), 1.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn add_v3_v3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a + b;
        assert_eq!(c.x(), 1.0 + 4.0);
        assert_eq!(c.y(), 2.0 + 5.0);
        assert_eq!(c.z(), 3.0 + 6.0);
    }

    #[test]
    fn add_assign_v3_v3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(a.x(), 1.0 + 4.0);
        assert_eq!(a.y(), 2.0 + 5.0);
        assert_eq!(a.z(), 3.0 + 6.0);
    }

    #[test]
    fn div_v3_v3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        let c = a / b;
        assert_float_eq!(c.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        let c = a / b;
        assert_float_eq!(c.x(), 1.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0 / 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn div_v3_f32() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        let c = a / b;
        assert_float_eq!(c.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        let c = a / b;
        assert_float_eq!(c.x(), 1.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0 / 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn div_assign_v3_v3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        a /= b;
        assert_float_eq!(a.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        a /= b;
        assert_float_eq!(a.x(), 1.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0 / 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn div_assign_v3_f32() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        a /= b;
        assert_float_eq!(a.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a /= b;
        assert_float_eq!(a.x(), 1.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0 / 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0 / 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn mul_v3_v3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        let c = a * b;
        assert_float_eq!(c.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        let c = a * b;
        assert_float_eq!(c.x(), 1.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0 * 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn mul_v3_f32() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        let c = a * b;
        assert_float_eq!(c.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        let c = a * b;
        assert_float_eq!(c.x(), 1.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0 * 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn mul_f32_v3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        let c = b * a;
        assert_float_eq!(c.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        let c = b * a;
        assert_float_eq!(c.x(), 1.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.y(), 2.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(c.z(), 3.0 * 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn mul_assign_v3_v3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        a *= b;
        assert_float_eq!(a.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        a *= b;
        assert_float_eq!(a.x(), 1.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0 * 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn mul_assign_v3_f32() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        a *= b;
        assert_float_eq!(a.x(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0, abs <= ABS_DIFF_LIMIT);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a *= b;
        assert_float_eq!(a.x(), 1.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 2.0 * 2.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 3.0 * 2.0, abs <= ABS_DIFF_LIMIT);
    }

    #[test]
    fn sub_v3_v3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a - b;
        assert_eq!(c.x(), 1.0 - 4.0);
        assert_eq!(c.y(), 2.0 - 5.0);
        assert_eq!(c.z(), 3.0 - 6.0);
    }

    #[test]
    fn sub_assign_v3_v3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        a -= b;
        assert_float_eq!(a.x(), 0.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.y(), 1.0, abs <= ABS_DIFF_LIMIT);
        assert_float_eq!(a.z(), 2.0, abs <= ABS_DIFF_LIMIT);
    }
}
