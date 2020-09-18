use crate::vec3::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
