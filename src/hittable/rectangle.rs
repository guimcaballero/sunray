use crate::{hittable::*, material::*, vec3::*};

pub struct XYRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub material: Material,
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;

        if t < t_min || t > t_max {
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

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.x0, self.y0, self.k - 0.0001),
            max: Point::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}

pub struct XZRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Material,
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;

        if t < t_min || t > t_max {
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

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.x0, self.z0, self.k - 0.0001),
            max: Point::new(self.x1, self.z1, self.k + 0.0001),
        })
    }
}

pub struct YZRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Material,
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;

        if t < t_min || t > t_max {
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

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Point::new(self.y0, self.z0, self.k - 0.0001),
            max: Point::new(self.y1, self.z1, self.k + 0.0001),
        })
    }
}
