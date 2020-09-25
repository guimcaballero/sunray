#![feature(clamp)]
#![feature(tau_constant)]

use rand::{rngs::*, Rng};
use rayon::prelude::*;
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
mod hit_record;
use hit_record::*;
mod hittable_list;
use hittable_list::*;
mod camera;
use camera::*;
mod material;
use material::*;
mod aabb;
use aabb::*;
mod bvh;
use bvh::*;

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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 50;
    let max_depth: u16 = 50;

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0; //(lookfrom - lookat).length();
    let vfov = 20.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // World
    let world = generate_world();

    let string = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            println!("Remaining scanlines: {}", j);
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _s in 0..samples_per_pixel {
                        let u =
                            (i as f64 + rand::thread_rng().gen::<f64>()) / (image_width - 1) as f64;
                        let v = (j as f64 + rand::thread_rng().gen::<f64>())
                            / (image_height - 1) as f64;

                        let ray = camera.ray(u, v);
                        pixel_color += ray_color(&ray, &world, max_depth);
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

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u16) -> Color {
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
            .scatter(&ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.3, 0.3, 1.0)
}

fn generate_world() -> HittableList {
    let mut world = HittableList::new();

    // Ground
    world.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Color::new(0.5, 0.5, 0.5)),
    }));

    // Spheres
    world.add(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    }));
    world.add(Box::new(Sphere {
        center: Point::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Color::new(0.4, 0.2, 0.1)),
    }));
    world.add(Box::new(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0),
    }));

    // Illuminated sphere
    world.add(Box::new(Sphere {
        center: Point::new(-4.0, 0.5, 2.0),
        radius: 0.5,
        material: Material::Lambertian(Color::new(2.0, 2.0, 1.0)),
    }));

    for a in -5..5 {
        for b in -5..5 {
            let choose_mat = rand::thread_rng().gen::<f64>();
            let center = Point {
                x: a as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
                y: 0.2,
                z: b as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
            };

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let center1 =
                        center + Vec3::new(0.0, rand::thread_rng().gen_range(0.0, 0.3), 0.0);
                    world.add(Box::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Material::Lambertian(albedo),
                    }));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(albedo, fuzz),
                    }));
                } else {
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    }));
                }
            }
        }
    }

    world
}
