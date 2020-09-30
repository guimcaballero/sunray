use crate::{hit_record::*, ray::*, texture::*, vec3::*};
use rand::*;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Material {
    Lambertian(Color),
    LambertianTexture(Texture),
    Metal(Color, f64),
    Dielectric(f64),
    DiffuseLight(Color),
    DiffuseLightTexture(Texture),
    Isotropic(Color),
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian(albedo) => {
                let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
                let ray = Ray {
                    origin: hit_record.point,
                    direction: scatter_direction,
                    time: ray_in.time,
                };
                *scattered = ray;
                *attenuation = albedo.clone();
                return true;
            }
            Self::LambertianTexture(albedo) => {
                let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
                let ray = Ray {
                    origin: hit_record.point,
                    direction: scatter_direction,
                    time: ray_in.time,
                };
                *scattered = ray;
                *attenuation = albedo(hit_record.u, hit_record.v, hit_record.point);
                return true;
            }
            Self::Metal(albedo, fuzz) => {
                let reflected = ray_in.direction.normalize().reflect(&hit_record.normal);
                let ray = Ray {
                    origin: hit_record.point,
                    direction: reflected + fuzz.min(1.0) * Vec3::random_in_unit_sphere(),
                    time: ray_in.time,
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
                        time: ray_in.time,
                    };
                    return true;
                }
                let reflect_prob = schlick(cos_theta, eta_over_etai);
                if rand::thread_rng().gen::<f64>() < reflect_prob {
                    let reflected = unit.reflect(&hit_record.normal);
                    *scattered = Ray {
                        origin: hit_record.point,
                        direction: reflected,
                        time: ray_in.time,
                    };
                    return true;
                }

                let refracted = unit.refract(&hit_record.normal, eta_over_etai);
                *scattered = Ray {
                    origin: hit_record.point,
                    direction: refracted,
                    time: ray_in.time,
                };

                return true;
            }
            Self::DiffuseLight(_) => {
                return false;
            }
            Self::DiffuseLightTexture(_) => {
                return false;
            }
            Self::Isotropic(albedo) => {
                *scattered = Ray {
                    origin: hit_record.point,
                    direction: Vec3::random_in_unit_sphere(),
                    time: ray_in.time,
                };
                *attenuation = albedo.clone();
                return true;
            }
        }
    }

    pub fn emitted(&self, u: f64, v: f64, point: Point) -> Color {
        match self {
            Self::DiffuseLight(emit) => *emit,
            Self::DiffuseLightTexture(emit) => emit(u, v, point),
            _ => Color::zeros(),
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
