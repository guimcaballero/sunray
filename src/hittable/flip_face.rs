use crate::hittable::*;

pub struct FlipFace {
    pub hittable: Box<dyn Hittable>,
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        if !self.hittable.hit(ray, taemin, t_max, hit_record) {
            return false;
        }

        hit_record.front_face = !hit_record.front_face;

        true
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
