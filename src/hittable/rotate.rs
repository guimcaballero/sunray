use crate::{hittable::*, vec3::*};

pub struct RotateY {
    pub hittable: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        if let Some(aabb) = hittable.bounding_box(0.0, 1.0) {
            let mut min = Vec3::infinity();
            let mut max = Vec3::neg_infinity();

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * aabb.max.x + (1 - i) as f64 * aabb.min.x;
                        let y = j as f64 * aabb.max.y + (1 - j) as f64 * aabb.min.y;
                        let z = k as f64 * aabb.max.z + (1 - k) as f64 * aabb.min.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            Self {
                hittable,
                sin_theta,
                cos_theta,
                aabb: Some(AABB { min, max }),
            }
        } else {
            Self {
                hittable,
                sin_theta,
                cos_theta,
                aabb: None,
            }
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let o_x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        let o_z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        let d_x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        let d_z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let rotated_ray = Ray {
            origin: Point::new(o_x, ray.origin.y, o_z),
            direction: Vec3::new(d_x, ray.direction.y, d_z),
            time: ray.time,
        };

        if !self.hittable.hit(&rotated_ray, t_min, t_max, hit_record) {
            return false;
        }

        hit_record.point.x =
            self.cos_theta * hit_record.point.x + self.sin_theta * hit_record.point.z;
        hit_record.point.z =
            -self.sin_theta * hit_record.point.x + self.cos_theta * hit_record.point.z;

        hit_record.normal.x =
            self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z;
        hit_record.normal.z =
            -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z;

        let normal = hit_record.normal;
        hit_record.set_face_normal(&rotated_ray, &normal);

        true
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.aabb
    }
}
