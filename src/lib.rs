use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f32, f32, f32);

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Vec3 {
    fn multiply(self, n: f32) -> Vec3 {
        Vec3 (self.0 * n, self.1 * n, self.2 * n)
    }

    fn divide(self, n: f32) -> Vec3 {
        Vec3 (self.0 / n, self.1 / n, self.2 / n)
    }

    fn dot(self, other: Vec3) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 (self.1*other.2 - self.2*other.1, 
            self.2*other.0 - self.0*other.2, 
            self.0*other.1 - self.1*other.0)
    }

    fn unit(self) -> Vec3 {
        self.divide(self.lenght())
    }

    fn lenght(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}