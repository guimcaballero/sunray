use crate::{hittable::*, material::*};

pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let center = self.center(ray.time);

        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if taemin < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material.clone();

                return true;
            }

            let temp = (-half_b + root) / a;
            if taemin < temp && temp < t_max {
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB {
            min: self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            min: self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        Some(box0.surrounding_box(box1))
    }
}

impl MovingSphere {
    pub fn center(&self, time: f32) -> Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}
