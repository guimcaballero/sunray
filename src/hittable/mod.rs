use crate::{aabb::*, hit_record::*, ray::*, vec3::*};

pub mod cube;
pub mod cylinder;
pub mod flip_face;
pub mod medium;
pub mod moving_sphere;
pub mod pyramid;
pub mod rectangle;
pub mod rotate;
pub mod sdf;
pub mod sphere;
pub mod translate;
pub mod triangle;

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    fn pdf_value(&self, point: &Point, vector: &Vec3) -> f32 {
        0.
    }
    fn random(&self, point: &Point) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
}
