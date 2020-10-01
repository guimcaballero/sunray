use crate::{hittable::*, material::*, vec3::*};

pub struct TracedSDF {
    pub sdf: Box<dyn SDF>,
    pub material: Material,
}

impl TracedSDF {
    pub fn normal(&self, point: Vec3) -> Vec3 {
        let h = 0.0001;

        let xyy = Vec3::new(1., -1., -1.);
        let yyx = Vec3::new(-1., -1., 1.);
        let yxy = Vec3::new(-1., 1., -1.);
        let xxx = Vec3::ones();

        (xyy * self.sdf.dist(point + xyy * h)
            + yyx * self.sdf.dist(point + yyx * h)
            + yxy * self.sdf.dist(point + yxy * h)
            + xxx * self.sdf.dist(point + xxx * h))
        .normalize()
    }
}

impl Hittable for TracedSDF {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut t = t_min;
        for _ in 0..200 {
            let point = ray.at(t);
            let distance = self.sdf.dist(point);

            if distance < 0.00001 {
                let normal = self.normal(point);
                *hit_record = HitRecord {
                    point,
                    normal,
                    t,
                    material: self.material.clone(),
                    ..*hit_record
                };
                hit_record.set_face_normal(&ray, &normal);

                return true;
            }
            if distance > 1000.0 || t > t_max {
                break;
            }

            t += distance;
        }

        false
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.sdf.bounding_box(t0, t1)
    }
}

pub trait SDF: Send + Sync {
    fn dist(&self, position: Vec3) -> f32;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub struct SDFSphere {
    pub radius: f32,
    pub center: Vec3,
}

impl SDF for SDFSphere {
    fn dist(&self, position: Vec3) -> f32 {
        (position - self.center).length() - self.radius
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Point::from(self.radius),
            max: self.center + Point::from(self.radius),
        })
    }
}

pub struct SDFCilinder {
    pub radius: f32,
    pub center: Vec3,
}

impl SDF for SDFCilinder {
    fn dist(&self, position: Vec3) -> f32 {
        Vec3::new(position.x - self.center.x, position.z - self.center.z, 0.).length() - self.radius
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Point::from(self.radius),
            max: self.center + Point::from(self.radius),
        })
    }
}

pub struct SDFDonut {
    pub radius0: f32,
    pub radius1: f32,
    pub center: Vec3,
}

impl SDF for SDFDonut {
    fn dist(&self, position: Vec3) -> f32 {
        let qx = Vec3::new(position.y - self.center.y, position.x - self.center.x, 0.0).length()
            - self.radius0;
        let qy = position.z - self.center.z;

        Vec3::new(qx, qy, 0.0).length() - self.radius1
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(
                -self.radius0 - self.radius1,
                -self.radius1,
                -self.radius0 - self.radius1,
            ),
            max: Point::new(
                self.radius0 + self.radius1,
                self.radius1,
                self.radius0 + self.radius1,
            ),
        })
    }
}
