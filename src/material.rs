use crate::{hit_record::*, onb::*, pdf::*, ray::*, texture::*, vec3::*};
use rand::*;
use std::f32::consts::PI;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Material {
    Normal,
    Lambertian(Color),
    LambertianTexture(Texture),
    Metal(Color, f32),
    Dielectric(f32),
    DiffuseLight(Color),
    DiffuseLightTexture(Texture),
    Isotropic(Color),
}

impl Material {
    pub fn emitted(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        u: f32,
        v: f32,
        point: Point,
    ) -> Color {
        if hit_record.front_face {
            match self {
                Self::Normal => hit_record.normal,
                Self::DiffuseLight(emit) => *emit,
                Self::DiffuseLightTexture(emit) => emit(u, v, point),
                _ => Color::zeros(),
            }
        } else {
            Color::zeros()
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian(albedo) => Some(ScatterRecord::Scatter {
                attenuation: *albedo,
                pdf: PDF::Cosine(ONB::build_from_w(hit_record.normal)),
            }),
            Self::LambertianTexture(albedo) => Some(ScatterRecord::Scatter {
                attenuation: albedo(hit_record.u, hit_record.v, hit_record.point),
                pdf: PDF::Cosine(ONB::build_from_w(hit_record.normal)),
            }),
            Self::Metal(albedo, fuzz) => {
                let reflected = ray_in.direction.reflect(&hit_record.normal);
                if reflected.x.is_nan() {
                    dbg!(ray_in.direction, hit_record.normal);
                    println!("mat");
                    std::process::exit(1);
                }
                Some(ScatterRecord::Specular {
                    specular_ray: Ray {
                        origin: hit_record.point,
                        direction: (reflected + *fuzz * Vec3::random_in_unit_sphere()).normalize(),
                        time: 0.,
                    },
                    attenuation: *albedo,
                })
            }
            Self::Dielectric(ref_idx) => {
                let attenuation = Color::ones();
                let eta_over_etai = if hit_record.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
                };
                let unit = ray_in.direction.normalize();

                let cos_theta = (-unit).dot(&hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                if eta_over_etai * sin_theta > 1.0 {
                    return Some(ScatterRecord::Specular {
                        specular_ray: Ray {
                            origin: hit_record.point,
                            direction: unit.reflect(&hit_record.normal),
                            time: ray_in.time,
                        },
                        attenuation,
                    });
                }
                let reflect_prob = schlick(cos_theta, eta_over_etai);
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    return Some(ScatterRecord::Specular {
                        specular_ray: Ray {
                            origin: hit_record.point,
                            direction: unit.reflect(&hit_record.normal),
                            time: ray_in.time,
                        },
                        attenuation,
                    });
                }

                Some(ScatterRecord::Specular {
                    specular_ray: Ray {
                        origin: hit_record.point,
                        direction: unit.refract(&hit_record.normal, eta_over_etai),
                        time: ray_in.time,
                    },
                    attenuation,
                })
            }
            Self::Isotropic(albedo) => Some(ScatterRecord::Specular {
                specular_ray: Ray {
                    origin: hit_record.point,
                    direction: Vec3::random_in_unit_sphere(),
                    time: ray_in.time,
                },
                attenuation: *albedo,
            }),
            _ => None,
        }
    }

    pub fn scattering_pdf(&self, _ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        match self {
            Self::Lambertian(_albedo) => {
                let cosine = hit_record.normal.dot(&scattered.direction.normalize());
                if cosine < 0. {
                    0.
                } else {
                    cosine / PI
                }
            }
            _ => 0.,
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub enum ScatterRecord<'a> {
    Specular {
        specular_ray: Ray,
        attenuation: Color,
    },
    Scatter {
        pdf: PDF<'a>,
        attenuation: Color,
    },
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lambertian(albedo) => f
                .debug_struct("Material::Lambertian")
                .field("albedo", albedo)
                .finish(),
            Self::LambertianTexture(_albedo) => {
                f.debug_struct("Material::LambertianTexture").finish()
            }
            Self::Metal(albedo, fuzz) => f
                .debug_struct("Material::Metal")
                .field("albedo", albedo)
                .field("fuzz", fuzz)
                .finish(),
            Self::Dielectric(idx) => f
                .debug_struct("Material::Dielectric")
                .field("ref_idx", idx)
                .finish(),
            Self::DiffuseLight(albedo) => f
                .debug_struct("Material::DiffuseLight")
                .field("albedo", albedo)
                .finish(),
            Self::DiffuseLightTexture(_albedo) => {
                f.debug_struct("Material::DiffuseLightTexture").finish()
            }
            Self::Isotropic(albedo) => f
                .debug_struct("Material::Isotropic")
                .field("albedo", albedo)
                .finish(),
            Self::Normal => f.debug_struct("Material::Normal").finish(),
        }
    }
}
