use crate::{
    hittable::{rectangle::*, *},
    hittable_list::*,
    material::*,
};

pub struct Cube {
    box_min: Point,
    box_max: Point,
    sides: HittableList,
}

impl Cube {
    pub fn new(box_min: Point, box_max: Point, material: Material) -> Self {
        let mut sides = HittableList::new();

        sides.add(Box::new(Rect {
            a0: box_min.x,
            a1: box_max.x,
            b0: box_min.y,
            b1: box_max.y,
            k: box_min.z,
            material: material.clone(),
            plane: Plane::XY,
        }));
        sides.add(Box::new(Rect {
            a0: box_min.x,
            a1: box_max.x,
            b0: box_min.y,
            b1: box_max.y,
            k: box_max.z,
            material: material.clone(),
            plane: Plane::XY,
        }));

        sides.add(Box::new(Rect {
            a0: box_min.x,
            a1: box_max.x,
            b0: box_min.z,
            b1: box_max.z,
            k: box_min.y,
            material: material.clone(),
            plane: Plane::XZ,
        }));
        sides.add(Box::new(Rect {
            a0: box_min.x,
            a1: box_max.x,
            b0: box_min.z,
            b1: box_max.z,
            k: box_max.y,
            material: material.clone(),
            plane: Plane::XZ,
        }));

        sides.add(Box::new(Rect {
            a0: box_min.y,
            a1: box_max.y,
            b0: box_min.z,
            b1: box_max.z,
            k: box_min.x,
            material: material.clone(),
            plane: Plane::YZ,
        }));
        sides.add(Box::new(Rect {
            a0: box_min.y,
            a1: box_max.y,
            b0: box_min.z,
            b1: box_max.z,
            k: box_max.x,
            material,
            plane: Plane::YZ,
        }));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(ray, taemin, t_max, hit_record)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.box_min,
            max: self.box_max,
        })
    }

    fn pdf_value(&self, point: &Point, vector: &Vec3) -> f32 {
        self.sides.pdf_value(point, vector)
    }

    fn random(&self, point: &Point) -> Vec3 {
        self.sides.random(point)
    }
}
