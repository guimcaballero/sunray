use crate::hittable::*;

pub struct RotateY {
    pub hittable: Box<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: Box<dyn Hittable>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        if let Some(aabb) = hittable.bounding_box(0.0, 1.0) {
            let mut min = Vec3::infinity();
            let mut max = Vec3::neg_infinity();

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * aabb.max.x + (1 - i) as f32 * aabb.min.x;
                        let y = j as f32 * aabb.max.y + (1 - j) as f32 * aabb.min.y;
                        let z = k as f32 * aabb.max.z + (1 - k) as f32 * aabb.min.z;

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
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        // Adapted from https://github.com/cbiffle/rtiow-rust/blob/master/src/object.rs#L349 because what I had before did weird stuff
        fn rot(p: Vec3, sin_theta: f32, cos_theta: f32) -> Vec3 {
            Vec3::new(
                p.dot(&Vec3::new(cos_theta, 0., sin_theta)),
                p.dot(&Vec3::new(0., 1., 0.)),
                p.dot(&Vec3::new(-sin_theta, 0., cos_theta)),
            )
        }

        let rot_ray = Ray {
            origin: rot(ray.origin, -self.sin_theta, self.cos_theta),
            direction: rot(ray.direction, -self.sin_theta, self.cos_theta),
            ..*ray
        };

        let mut temp_rec = HitRecord::default();
        if self.hittable.hit(&rot_ray, taemin, t_max, &mut temp_rec) {
            *hit_record = HitRecord {
                point: rot(temp_rec.point, self.sin_theta, self.cos_theta),
                normal: rot(temp_rec.normal, self.sin_theta, self.cos_theta),
                ..temp_rec
            };
            return true;
        }
        false
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.aabb
    }
}
