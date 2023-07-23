use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Debug)]
struct Vec3(f32, f32, f32);

#[allow(dead_code)]
impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { 0: x, 1: y, 2: z }
    }

    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }

    fn z(&self) -> f32 {
        self.2
    }

    fn length_squared(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn dot(&self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn cross(&self, other: Self) -> Self {
        Self {
            0: self.1 * other.2 - self.2 * other.1,
            1: self.2 * other.0 - self.0 * other.2,
            2: self.0 * other.1 - self.1 * other.0,
        }
    }

    fn unit_vec3(&self) -> Self {
        self / self.length()
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            0: self.0 * rhs,
            1: self.1 * rhs,
            2: self.2 * rhs,
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            0: self.0 / rhs,
            1: self.1 / rhs,
            2: self.2 / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            0: -self.0,
            1: -self.1,
            2: -self.2,
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            0: value.0,
            1: value.1,
            2: value.2,
        }
    }
}
