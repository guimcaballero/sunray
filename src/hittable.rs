use crate::{aabb::*, hit_record::*, material::*, ray::*, vec3::*};
use std::f64::consts::PI;

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if t_min < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material.clone();
                let (u, v) = get_sphere_uv((hit_record.point - self.center) / self.radius);
                hit_record.u = u;
                hit_record.v = v;

                return true;
            }

            let temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material.clone();
                let (u, v) = get_sphere_uv((hit_record.point - self.center) / self.radius);
                hit_record.u = u;
                hit_record.v = v;

                return true;
            }
        }

        false
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        true
    }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();

    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let center = self.center(ray.time);

        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if t_min < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material.clone();

                return true;
            }

            let temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material.clone();

                return true;
            }
        }

        false
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB {
            min: self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            min: self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        *output_box = box0.surrounding_box(box1);

        true
    }
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

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
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            min: Point::new(self.x0, self.y0, self.k - 0.0001),
            max: Point::new(self.x1, self.y1, self.k + 0.0001),
        };
        true
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
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            min: Point::new(self.x0, self.z0, self.k - 0.0001),
            max: Point::new(self.x1, self.z1, self.k + 0.0001),
        };
        true
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
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            min: Point::new(self.y0, self.z0, self.k - 0.0001),
            max: Point::new(self.y1, self.z1, self.k + 0.0001),
        };
        true
    }
}
