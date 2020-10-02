use crate::{aabb::*, hit_record::*, ray::*};

pub mod cube;
pub mod medium;
pub mod moving_sphere;
pub mod rectangle;
pub mod rotate;
pub mod sdf;
pub mod sphere;
pub mod translate;

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
