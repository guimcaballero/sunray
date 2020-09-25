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

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(&ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::default();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(t0, t1, &mut temp_box) {
                return false;
            }

            *output_box = if first_box {
                temp_box
            } else {
                output_box.surrounding_box(temp_box)
            };

            first_box = false;
        }

        return true;
    }
}
