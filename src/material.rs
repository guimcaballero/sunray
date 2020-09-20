use crate::{hittable::*, ray::*, vec3::*};
use rand::*;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
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
            Self::Metal(albedo, fuzz) => {
                let reflected = ray_in.direction.normalize().reflect(&hit_record.normal);
                let ray = Ray {
                    origin: hit_record.point,
                    direction: reflected + fuzz.min(1.0) * Vec3::random_in_unit_sphere(rng),
                };
                *scattered = ray;
                *attenuation = albedo.clone();
                return reflected.dot(&hit_record.normal) > 0.0;
            }
            Self::Dielectric(ref_idx) => {
                *attenuation = Color::ones();
                let eta_over_etai = if hit_record.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
                };
                let unit = ray_in.direction.normalize();

                let cos_theta = (-unit).dot(&hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                if eta_over_etai * sin_theta > 1.0 {
                    let reflected = unit.reflect(&hit_record.normal);
                    *scattered = Ray {
                        origin: hit_record.point,
                        direction: reflected,
                    };
                    return true;
                }
                let reflect_prob = schlick(cos_theta, eta_over_etai);
                if rng.gen::<f64>() < reflect_prob {
                    let reflected = unit.reflect(&hit_record.normal);
                    *scattered = Ray {
                        origin: hit_record.point,
                        direction: reflected,
                    };
                    return true;
                }

                let refracted = unit.refract(&hit_record.normal, eta_over_etai);
                *scattered = Ray {
                    origin: hit_record.point,
                    direction: refracted,
                };

                return true;
            }
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
