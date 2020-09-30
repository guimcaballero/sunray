use crate::{aabb::*, hit_record::*, ray::*};

pub mod cube;
pub mod moving_sphere;
pub mod rectangle;
pub mod rotate;
pub mod sphere;
pub mod translate;

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
