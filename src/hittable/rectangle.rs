use crate::{hittable::*, material::*};
use rand::Rng;

#[derive(Clone)]
pub enum Plane {
    XY,
    XZ,
    YZ,
}

impl Plane {
    // From https://github.com/fralken/ray-tracing-the-rest-of-your-life/blob/master/src/rect.rs
    fn get_index(&self) -> (usize, usize, usize) {
        match self {
            Plane::XY => (2, 0, 1),
            Plane::XZ => (1, 0, 2),
            Plane::YZ => (0, 1, 2),
        }
    }
}

pub struct Rect {
    pub a0: f32,
    pub a1: f32,
    pub b0: f32,
    pub b1: f32,
    pub k: f32,
    pub material: Material,
    pub plane: Plane,
}

impl Hittable for Rect {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let (k_index, a_index, b_index) = self.plane.get_index();

        let t = (self.k - ray.origin[k_index]) / ray.direction[k_index];

        if t < taemin || t > t_max {
            return false;
        }

        let a = ray.origin[a_index] + t * ray.direction[a_index];
        let b = ray.origin[b_index] + t * ray.direction[b_index];

        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return false;
        }

        hit_record.u = (a - self.a0) / (self.a1 - self.a0);
        hit_record.v = (b - self.b0) / (self.b1 - self.b0);
        hit_record.t = t;

        let mut normal = Vec3::zeros();
        normal[k_index] = 1.0;
        hit_record.set_face_normal(ray, &normal);

        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.a0, self.b0, self.k - 0.0001),
            max: Point::new(self.a1, self.b1, self.k + 0.0001),
        })
    }

    fn pdf_value(&self, point: &Point, vector: &Vec3) -> f32 {
        let mut hit_record = HitRecord::default();
        let ray = Ray {
            origin: *point,
            direction: *vector,
            time: 0.,
        };
        if !self.hit(&ray, 0.001, f32::INFINITY, &mut hit_record) {
            return 0.;
        }

        let area = (self.a1 - self.a0) * (self.b1 - self.b0);
        let distance_squared = hit_record.t.powi(2) * vector.length_squared();
        let cosine = vector.dot(&hit_record.normal).abs() / vector.length();
        if cosine != 0.0 {
            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, point: &Point) -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut random_point = Vec3::zeros();

        let (k_axis, a_axis, b_axis) = self.plane.get_index();
        random_point[a_axis] = rng.gen_range(&self.a0, &self.a1);
        random_point[b_axis] = rng.gen_range(&self.b0, &self.b1);
        random_point[k_axis] = self.k;

        random_point - *point
    }
}
