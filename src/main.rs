#![feature(clamp)]
#![feature(tau_constant)]

use rand::{rngs::*, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::time::Instant;

mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
use hittable::*;
mod hittable_list;
use hittable_list::*;
mod camera;
use camera::*;
mod material;
use material::*;

fn main() {
    let start = Instant::now();

    let path = Path::new("image.ppm");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let result = get_image_string();

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    println!("{:.2?} seconds to run.", start.elapsed());
}

fn get_image_string() -> String {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u8;
    let samples_per_pixel = 100;

    // Camera
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();
    let sphere = Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Lambertian(Color::new(0.7, 0.3, 0.3)),
    };
    let left_sphere = Sphere {
        center: Point::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal(Color::new(0.8, 0.8, 0.8)),
    };
    let right_sphere = Sphere {
        center: Point::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal(Color::new(0.8, 0.6, 0.2)),
    };
    let big_sphere = Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian(Color::new(0.8, 0.8, 0.0)),
    };
    world.add(&sphere);
    world.add(&left_sphere);
    world.add(&right_sphere);
    world.add(&big_sphere);

    let mut rng = rand::thread_rng();
    let max_depth = 50;

    let mut result = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        println!("Remaining scanlines: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = camera.ray(u, v);
                pixel_color += ray_color(&ray, &world, &mut rng, max_depth);
            }

            pixel_color.write_color(samples_per_pixel, &mut result);
        }
    }

    return result;
}

fn ray_color(ray: &Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: u8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = HitRecord::default();
    if world.hit(&ray, 0.001, f64::INFINITY, &mut hit_record) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        if hit_record
            .material
            .scatter(&ray, &hit_record, &mut attenuation, &mut scattered, rng)
        {
            return attenuation * ray_color(&scattered, world, rng, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.3, 0.3, 1.0)
}
