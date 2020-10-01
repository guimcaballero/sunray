use crate::{hittable::*, material::*, vec3::*};
use rand::Rng;

pub struct ConstantMedium {
    pub hittable: Box<dyn Hittable>,
    pub phase_function: Material,
    pub neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(hittable: Box<dyn Hittable>, density: f32, color: Color) -> Self {
        Self {
            hittable,
            neg_inv_density: (-1.0 / density),
            phase_function: Material::Isotropic(color),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self
            .hittable
            .hit(ray, -f32::INFINITY, f32::INFINITY, &mut rec1)
        {
            return false;
        }
        if !self
            .hittable
            .hit(ray, rec1.t + 0.0001, f32::INFINITY, &mut rec2)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min
        }
        if rec2.t > t_max {
            rec2.t = t_max
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::thread_rng().gen::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        hit_record.t = rec1.t + hit_distance / ray_length;
        hit_record.point = ray.at(hit_record.t);

        hit_record.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        hit_record.front_face = true; // also arbitrary
        hit_record.material = self.phase_function.clone();

        true
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
