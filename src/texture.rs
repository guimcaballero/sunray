use crate::{perlin::Perlin, vec3::*};
use std::sync::Arc;

pub type Texture = Arc<dyn Fn(f64, f64, Vec3) -> Vec3 + Send + Sync>;

pub fn solid_color(color: Vec3) -> Texture {
    Arc::new(move |_, _, _| color)
}

pub fn checker(even: Texture, odd: Texture) -> Texture {
    Arc::new(move |u, v, p| {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            odd(u, v, p)
        } else {
            even(u, v, p)
        }
    })
}

pub fn noise(perlin: Perlin, scale: f64) -> Texture {
    Arc::new(move |_, _, p| Color::ones() * 0.5 * (1.0 + perlin.noise(scale * p)))
}

#[allow(dead_code)]
pub fn turbulent_noise(perlin: Perlin, scale: f64) -> Texture {
    Arc::new(move |_, _, p| Color::ones() * perlin.turbulence(scale * p, 7))
}

pub fn marble(perlin: Perlin, scale: f64) -> Texture {
    Arc::new(move |_, _, p| {
        Color::ones()
            * 0.5
            * (1.0
                + (scale * (p.x % 13.0 + 2.0) * (p.z % 7.0 + 3.0) * 0.05
                    + 10.0 * perlin.turbulence(p, 7))
                .sin())
    })
}
