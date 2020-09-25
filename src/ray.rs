use crate::vec3::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
            time: 0.0,
        }
    }
}
