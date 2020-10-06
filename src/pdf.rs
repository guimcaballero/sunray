use crate::{hittable::*, onb::*, vec3::*};
use rand::Rng;
use std::f32::consts::PI;

pub enum PDF {
    Cosine(ONB),
    Hittable {
        hittable: Box<dyn Hittable>,
        origin: Point,
    },
    Mixture {
        p: Box<PDF>,
        q: Box<PDF>,
    },
}

impl PDF {
    pub fn value(&self, direction: Vec3) -> f32 {
        match self {
            Self::Cosine(uvw) => {
                let cosine = direction.normalize().dot(&uvw.w);
                if cosine <= 0. {
                    0.
                } else {
                    cosine / PI
                }
            }
            Self::Hittable { hittable, origin } => hittable.pdf_value(origin, &direction),
            Self::Mixture { p, q } => 0.5 * p.value(direction) + 0.5 * q.value(direction),
        }
    }

    pub fn generate(&self) -> Vec3 {
        match self {
            Self::Cosine(uvw) => uvw.local(Vec3::random_cosine_direction()),
            Self::Hittable { hittable, origin } => hittable.random(origin),
            Self::Mixture { p, q } => {
                if rand::thread_rng().gen::<f32>() < 0.5 {
                    p.generate()
                } else {
                    q.generate()
                }
            }
        }
    }
}
