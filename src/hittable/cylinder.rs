use crate::{hittable::*, material::*, onb::*};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Cylinder {
    pub center: Point,
    pub radius: f32,
    // pub axis: Vec3,
    // TODO Need to reimplement axis. Currently only works for vertical axis
    pub material: Material,
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let axis = Vec3::new(0., 1., 0.);
        let oc = ray.origin - self.center;
        let card = axis.dot(&ray.direction);
        let caoc = axis.dot(&ray.origin);
        let a = 1.0 - card * card;
        let b = oc.dot(&ray.direction) - caoc * card;
        let c = oc.length_squared() - caoc * caoc - self.radius * self.radius;
        let h = b * b - a * c;

        if h < 0.0 {
            return false;
        }

        let h = h.sqrt();

        let temp = (-b - h) / a;
        if taemin < temp && temp < t_max {
            hit_record.t = temp;
            hit_record.point = ray.at(temp);

            let outward_normal = {
                let new_center = Point::new(self.center.x, hit_record.point.y, self.center.z);
                (hit_record.point - new_center).normalize()
            };

            hit_record.set_face_normal(&ray, &outward_normal);
            hit_record.material = self.material.clone();

            return true;
        }

        let temp = (-b + h) / a;
        if taemin < temp && temp < t_max {
            hit_record.t = temp;
            hit_record.point = ray.at(temp);

            let outward_normal = {
                let new_center = Point::new(self.center.x, hit_record.point.y, self.center.z);
                (hit_record.point - new_center).normalize()
            };

            hit_record.set_face_normal(&ray, &outward_normal);
            hit_record.material = self.material.clone();

            return true;
        }

        false
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        None
    }

    fn pdf_value(&self, point: &Point, vector: &Vec3) -> f32 {
        // TODO Fix this

        let mut hit_record = HitRecord::default();
        let ray = Ray {
            origin: *point,
            direction: *vector,
            time: 0.,
        };
        if !self.hit(&ray, 0.001, f32::INFINITY, &mut hit_record) {
            return 0.;
        }

        let cos_theta_max =
            (1. - self.radius * self.radius / (self.center - *point).length_squared()).sqrt();
        let solid_angle = 2. * PI * (1. - cos_theta_max);

        1. / solid_angle
    }

    fn random(&self, point: &Point) -> Vec3 {
        let direction = self.center - *point;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(direction);
        uvw.local(Vec3::random_to_sphere(self.radius, distance_squared))
    }
}
