#![feature(clamp, box_syntax)]

use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

mod onb;
mod pdf;
use pdf::*;
mod perlin;
mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
use hittable::*;
mod hit_record;
use hit_record::*;
mod aabb;
mod bvh;
mod camera;
mod hittable_list;
mod material;
use material::ScatterRecord;
mod scenes;
mod texture;
use scenes::*;

fn main() {
    let start = Instant::now();

    let path = Path::new("image.ppm");
    let display = path.display();

    let result = get_image_string();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // Yeah it's a bit dumb to open the image again
    let img = image::open("image.ppm").unwrap();
    img.save("image.png").unwrap();

    println!("{:.2?} seconds to run.", start.elapsed());
}

fn get_image_string() -> String {
    // World
    let World {
        hittables,
        camera,
        lights,
        background_color_top,
        background_color_bottom,
        samples_per_pixel,
        image_width,
        aspect_ratio,
        max_depth,
    } = scenes::generate_world();

    let image_height = (image_width as f32 / aspect_ratio) as u16;

    println!(
        "Rendering {}x{} with {} samples",
        image_width, image_height, samples_per_pixel
    );

    let string = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            println!("Remaining scanlines: {}", j);
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    let mut rng = rand::thread_rng();
                    for _s in 0..samples_per_pixel {
                        let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                        let ray = camera.ray(u, v);
                        pixel_color += ray_color(
                            &ray,
                            background_color_top,
                            background_color_bottom,
                            &hittables,
                            &*lights,
                            max_depth,
                        );
                    }

                    pixel_color.write_color(samples_per_pixel)
                })
                .collect()
        })
        .map(|array: Vec<String>| array.join(""))
        .collect::<Vec<String>>()
        .join("");

    format!("P3\n{} {}\n255\n{}", image_width, image_height, string)
}

fn ray_color(
    ray: &Ray,
    background_color_top: Color,
    background_color_bottom: Color,
    hittables: &dyn Hittable,
    lights: &dyn Hittable,
    depth: u16,
) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zeros();
    }

    let mut hit_record = HitRecord::default();
    if !hittables.hit(&ray, 0.001, f32::INFINITY, &mut hit_record) {
        let t = 0.5 * (ray.direction.normalize().y + 1.0);
        // return (1.0 - t) * Color::new(0.05, 0.05, 0.2) + t * Color::zeros();
        return (1.0 - t) * background_color_bottom + t * background_color_top;
    }

    let emitted = hit_record.material.emitted(
        ray,
        &hit_record,
        hit_record.u,
        hit_record.v,
        hit_record.point,
    );
    if let Some(srec) = hit_record.material.scatter(ray, &hit_record) {
        match srec {
            ScatterRecord::Scatter { pdf, attenuation } => {
                let light_pdf = PDF::Hittable {
                    hittable: lights,
                    origin: hit_record.point,
                };
                let p = PDF::Mixture {
                    p: box light_pdf,
                    q: box pdf,
                };

                let scattered = Ray {
                    origin: hit_record.point,
                    direction: p.generate(),
                    time: ray.time,
                };
                let pdf_val = p.value(scattered.direction);

                emitted
                    + attenuation
                        * hit_record
                            .material
                            .scattering_pdf(ray, &hit_record, &scattered)
                        * ray_color(
                            &scattered,
                            background_color_top,
                            background_color_bottom,
                            hittables,
                            lights,
                            depth - 1,
                        )
                        / pdf_val
            }
            ScatterRecord::Specular {
                specular_ray,
                attenuation,
            } => {
                attenuation
                    * ray_color(
                        &specular_ray,
                        background_color_top,
                        background_color_bottom,
                        hittables,
                        lights,
                        depth - 1,
                    )
            }
        }
    } else {
        emitted
    }
}
