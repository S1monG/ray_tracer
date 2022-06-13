use super::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f32) -> Ray {
        self.origin + self.direction.multiply(t)
    }
}