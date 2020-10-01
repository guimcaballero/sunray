use crate::{hittable::*, vec3::*};

pub struct Translate {
    pub hittable: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Box<dyn Hittable>, offset: Vec3) -> Self {
        Self { hittable, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let moved = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };

        if !self.hittable.hit(&moved, t_min, t_max, hit_record) {
            return false;
        }

        hit_record.point += self.offset;
        let normal = hit_record.normal;
        hit_record.set_face_normal(&moved, &normal);

        true
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(output_box) = self.hittable.bounding_box(t0, t1) {
            Some(AABB {
                min: output_box.min + self.offset,
                max: output_box.max + self.offset,
            })
        } else {
            None
        }
    }
}
