use crate::vec3::*;
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
