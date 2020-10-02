#![feature(clamp)]

use image;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

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
mod scenes;
mod texture;
use scenes::*;

const SCENE: Scene = Scene::FinalScene;

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
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 10000;
    let max_depth: u16 = 50;

    // World
    let World {
        hittables,
        camera,
        background_color,
    } = scenes::generate_world(SCENE, aspect_ratio);

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
                        pixel_color += ray_color(&ray, background_color, &hittables, max_depth);
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

fn ray_color(ray: &Ray, background_color: Color, hittables: &dyn Hittable, depth: u16) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = HitRecord::default();
    if !hittables.hit(&ray, 0.001, f32::INFINITY, &mut hit_record) {
        return background_color;
    }

    let mut scattered = Ray::default();
    let mut attenuation = Color::new(0.0, 0.0, 0.0);
    let emitted = hit_record
        .material
        .emitted(hit_record.u, hit_record.v, hit_record.point);

    if !hit_record
        .material
        .scatter(&ray, &hit_record, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenuation * ray_color(&scattered, background_color, hittables, depth - 1)
}
