use crate::{hittable::*, material::*, vec3::*};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
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
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();

    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}
