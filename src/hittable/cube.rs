use crate::{
    hittable::{rectangle::*, *},
    hittable_list::*,
    material::*,
    vec3::*,
};

pub struct Cube {
    box_min: Point,
    box_max: Point,
    sides: HittableList,
}

impl Cube {
    pub fn new(box_min: Point, box_max: Point, material: Material) -> Self {
        let mut sides = HittableList::new();

        sides.add(Box::new(XYRect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_min.z,
            material: material.clone(),
        }));
        sides.add(Box::new(XYRect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_max.z,
            material: material.clone(),
        }));

        sides.add(Box::new(XZRect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.y,
            material: material.clone(),
        }));
        sides.add(Box::new(XZRect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.y,
            material: material.clone(),
        }));

        sides.add(Box::new(YZRect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.x,
            material: material.clone(),
        }));
        sides.add(Box::new(YZRect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.x,
            material: material.clone(),
        }));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, hit_record)
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.box_min,
            max: self.box_max,
        })
    }
}
