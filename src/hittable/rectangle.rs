use crate::{hittable::*, material::*};
use rand::Rng;

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;

        if t < taemin || t > t_max {
            return false;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (y - self.y0) / (self.y1 - self.y0);
        hit_record.t = t;
        hit_record.set_face_normal(ray, &Vec3::new(0.0, 0.0, 1.0));
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.x0, self.y0, self.k - 0.0001),
            max: Point::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}

pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;

        if t < taemin || t > t_max {
            return false;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        hit_record.set_face_normal(ray, &Vec3::new(0.0, 1.0, 0.0));
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.x0, self.z0, self.k - 0.0001),
            max: Point::new(self.x1, self.z1, self.k + 0.0001),
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

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = hit_record.t * hit_record.t * vector.length_squared();
        let cosine = (vector.dot(&hit_record.normal) / vector.length()).abs();

        distance_squared / (cosine * area)
    }
    fn random(&self, point: &Point) -> Vec3 {
        let mut rng = rand::thread_rng();
        let random_point = Point::new(
            rng.gen_range(self.x0, self.x1),
            self.k,
            rng.gen_range(self.z0, self.z1),
        );
        random_point - *point
    }
}

pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;

        if t < taemin || t > t_max {
            return false;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        hit_record.u = (y - self.y0) / (self.y1 - self.y0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        hit_record.set_face_normal(ray, &Vec3::new(1.0, 0.0, 0.0));
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.y0, self.z0, self.k - 0.0001),
            max: Point::new(self.y1, self.z1, self.k + 0.0001),
        })
    }
}
