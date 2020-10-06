use crate::{perlin::Perlin, vec3::*};
use image as img;
use std::sync::Arc;

pub type Texture = Arc<dyn Fn(f32, f32, Vec3) -> Vec3 + Send + Sync>;

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

// Perlin is a param, instead of being initialized inside the function so that we can reuse it when making different textures

pub fn noise(perlin: Perlin, scale: f32) -> Texture {
    Arc::new(move |_, _, p| Color::ones() * 0.5 * (1.0 + perlin.noise(scale * p)))
}
#[allow(dead_code)]
pub fn turbulent_noise(perlin: Perlin, scale: f32) -> Texture {
    Arc::new(move |_, _, p| Color::ones() * perlin.turbulence(scale * p, 7))
}
pub fn marble(perlin: Perlin, scale: f32) -> Texture {
    Arc::new(move |_, _, p| {
        Color::ones()
            * 0.5
            * (1.0
                + (scale * (p.x % 13.0 + 2.0) * (p.z % 7.0 + 3.0) * 0.05
                    + 10.0 * perlin.turbulence(p, 7))
                .sin())
    })
}

pub fn image(filename: &str) -> Texture {
    let image = img::open(filename).expect("Image doesn't exist").to_rgb();
    let (width, height) = image.dimensions();
    let width = width as usize;
    let height = height as usize;

    let data = image.into_raw();

    Arc::new(move |u, v, _| {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * width as f32) as usize;
        let mut j = (v * height as f32) as usize;

        if i >= width {
            i = width - 1;
        }
        if j >= height {
            j = height - 1;
        }

        let index = 3 * i + 3 * width * j;
        Color::new(
            data[index] as f32 / 255.0,
            data[index + 1] as f32 / 255.0,
            data[index + 2] as f32 / 255.0,
        )
    })
}
