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

#[cfg(target_arch = "wasm32")]
pub use renderer::get_image_ppm_single_threaded;

pub use renderer::get_image_ppm;
pub use scenes::Scene;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
