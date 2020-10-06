use crate::{hittable::*, material::*};

#[derive(Clone)]
pub struct Cylinder {
    pub center: Point,
    pub radius: f32,
    pub axis: Vec3,
    pub material: Material,
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let card = self.axis.dot(&ray.direction);
        let caoc = self.axis.dot(&ray.origin);
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
            let outward_normal = (hit_record.point - self.center) / self.radius;
            hit_record.set_face_normal(&ray, &outward_normal);
            hit_record.material = self.material.clone();
        }

        let temp = (-b + h) / a;
        if taemin < temp && temp < t_max {
            hit_record.t = temp;
            hit_record.point = ray.at(temp);
            let outward_normal = (hit_record.point - self.center) / self.radius;
            hit_record.set_face_normal(&ray, &outward_normal);
            hit_record.material = self.material.clone();
        }

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        None
    }
}
