use crate::{hittable::*, ray::*, vec3::*};
use rand::*;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color),
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut rngs::ThreadRng,
    ) -> bool {
        match self {
            Self::Lambertian(albedo) => {
                let scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);
                let ray = Ray {
                    origin: hit_record.point,
                    direction: scatter_direction,
                };
                *scattered = ray;
                *attenuation = albedo.clone();
                return true;
            }
            Self::Metal(albedo) => {
                let reflected = ray_in.direction.normalize().reflect(&hit_record.normal);
                let ray = Ray {
                    origin: hit_record.point,
                    direction: reflected,
                };
                *scattered = ray;
                *attenuation = albedo.clone();
                return reflected.dot(&hit_record.normal) > 0.0;
            }
        }
    }
}
