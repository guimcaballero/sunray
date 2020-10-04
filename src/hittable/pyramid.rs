use crate::{
    hittable::{triangle::*, *},
    hittable_list::*,
    material::*,
    vec3::*,
};

pub struct Pyramid {
    top: Point,
    base0: Point,
    base1: Point,
    base2: Point,
    base3: Point,
    sides: HittableList,
}

impl Pyramid {
    pub fn new(
        top: Point,
        base0: Point,
        base1: Point,
        base2: Point,
        base3: Point,
        material: Material,
    ) -> Self {
        let mut sides = HittableList::new();

        // Base
        sides.add(Box::new(Triangle {
            v0: base0,
            v1: base1,
            v2: base2,
            material: material.clone(),
        }));
        sides.add(Box::new(Triangle {
            v0: base0,
            v1: base2,
            v2: base3,
            material: material.clone(),
        }));

        sides.add(Box::new(Triangle {
            v0: top,
            v1: base0,
            v2: base1,
            material: material.clone(),
        }));
        sides.add(Box::new(Triangle {
            v0: top,
            v1: base1,
            v2: base2,
            material: material.clone(),
        }));
        sides.add(Box::new(Triangle {
            v0: top,
            v1: base2,
            v2: base3,
            material: material.clone(),
        }));
        sides.add(Box::new(Triangle {
            v0: top,
            v1: base3,
            v2: base0,
            material: material.clone(),
        }));

        Self {
            top,
            base0,
            base1,
            base2,
            base3,
            sides,
        }
    }
}

impl Hittable for Pyramid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, hit_record)
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(
                self.top
                    .x
                    .min(self.base0.x)
                    .min(self.base1.x)
                    .min(self.base2.x)
                    .min(self.base3.x),
                self.top
                    .y
                    .min(self.base0.y)
                    .min(self.base1.y)
                    .min(self.base2.y)
                    .min(self.base3.y),
                self.top
                    .z
                    .min(self.base0.z)
                    .min(self.base1.z)
                    .min(self.base2.z)
                    .min(self.base3.z),
            ),
            max: Point::new(
                self.top
                    .x
                    .max(self.base0.x)
                    .max(self.base1.x)
                    .max(self.base2.x)
                    .max(self.base3.x),
                self.top
                    .y
                    .max(self.base0.y)
                    .max(self.base1.y)
                    .max(self.base2.y)
                    .max(self.base3.y),
                self.top
                    .z
                    .max(self.base0.z)
                    .max(self.base1.z)
                    .max(self.base2.z)
                    .max(self.base3.z),
            ),
        })
    }
}
