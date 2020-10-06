use crate::{aabb::*, hit_record::*, hittable::*, ray::*};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl<'a> HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable for HittableList {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::default();
            if object.hit(&ray, taemin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp_box = AABB::default();

        for object in &self.objects {
            if let Some(bbox) = object.bounding_box(t0, t1) {
                temp_box = temp_box.surrounding_box(bbox);
            } else {
                return None;
            }
        }

        Some(temp_box)
    }
}
