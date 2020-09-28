use crate::{material::*, ray::*, texture, vec3::*};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            front_face: false,
            material: Material::Lambertian(Color::zeros()),
            u: 0.0,
            v: 0.0,
        }
    }
}
