use crate::{aabb::*, hit_record::*, hittable::*, ray::*};
use rand::*;
use std::cmp::Ordering;

#[allow(dead_code)]
pub struct BVHNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bbox: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }

        let mut hit_left = false;
        if let Some(left) = &self.left {
            hit_left = left.hit(ray, t_min, t_max, hit_record);
        }
        let mut hit_right = false;
        if let Some(right) = &self.left {
            hit_right = right.hit(ray, t_min, t_max, hit_record);
        }

        hit_left || hit_right
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

impl BVHNode {
    #[allow(dead_code)]
    pub fn new(
        objects: &mut Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = rand::thread_rng().gen_range(0, 2);

        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            // TODO Here we don't have the same item twice, because we're removing it
            ((objects.remove(start)), objects.remove(start))
        } else if object_span == 2 {
            match comparator(&*objects[start], &*objects[start + 1]) {
                Ordering::Greater => ((objects.remove(start)), objects.remove(start + 1)),
                _ => ((objects.remove(start + 1)), objects.remove(start)),
            }
        } else {
            objects.sort_by(|a, b| comparator(&**a, &**b));
            let mid = start + object_span / 2;

            let left_box = BVHNode::new(objects, start, mid, time0, time1);
            let right_box = BVHNode::new(objects, mid, end, time0, time1);

            (
                Box::new(left_box) as Box<dyn Hittable>,
                Box::new(right_box) as Box<dyn Hittable>,
            )
        };

        if let Some(box_left) = left.bounding_box(time0, time1) {
            if let Some(box_right) = right.bounding_box(time0, time1) {
                return Self {
                    left: Some(left),
                    right: Some(right),
                    bbox: box_left.surrounding_box(box_right),
                };
            }
        }
        panic!("No bounding box in bvh_node constructor.");
    }
}

fn box_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable, axis: u16) -> Ordering {
    if let Some(box_a) = a.bounding_box(0.0, 0.0) {
        if let Some(box_b) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_a.min[axis].partial_cmp(&box_b.min[axis]) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in bvh_node constructor.");
}

#[allow(dead_code)]
fn box_x_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable) -> Ordering {
    box_compare(a, b, 0)
}

#[allow(dead_code)]
fn box_y_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable) -> Ordering {
    box_compare(a, b, 1)
}

#[allow(dead_code)]
fn box_z_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable) -> Ordering {
    box_compare(a, b, 2)
}
