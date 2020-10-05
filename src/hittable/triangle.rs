use crate::{hittable::*, material::*, vec3::*};

#[derive(Clone)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub material: Material,
}

impl Hittable for Triangle {
    #[allow(unused_variables)]
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin - self.v0;

        let normal = v1v0.cross(&v2v0);
        let q = rov0.cross(&ray.direction);
        let d_inv = ray.direction.dot(&normal);

        let u = (-q).dot(&v2v0) / d_inv;
        let v = q.dot(&v1v0) / d_inv;
        let t = (-normal).dot(&rov0) / d_inv;

        if u < 0.0 || u > 1.0 || v < 0.0 || (u + v) > 1.0 {
            return false;
        }

        // Drip drop out of here if we're out of bounds
        if taemin > t || t > t_max {
            return false;
        }

        hit_record.t = t;
        hit_record.point = ray.at(t);
        hit_record.set_face_normal(&ray, &normal.normalize());
        hit_record.material = self.material.clone();
        hit_record.u = u;
        hit_record.v = v;

        true
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(
                self.v0.x.min(self.v1.x).min(self.v2.x),
                self.v0.y.min(self.v1.y).min(self.v2.y),
                self.v0.z.min(self.v1.z).min(self.v2.z),
            ),
            max: Point::new(
                self.v0.x.max(self.v1.x).max(self.v2.x),
                self.v0.y.max(self.v1.y).max(self.v2.y),
                self.v0.z.max(self.v1.z).max(self.v2.z),
            ),
        })
    }
}
