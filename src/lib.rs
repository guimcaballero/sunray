#![feature(clamp, box_syntax)]

mod aabb;
mod bvh;
mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod material;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod renderer;
mod scenes;
mod texture;
mod vec3;

pub use renderer::get_image_ppm;
pub use scenes::Scene;
