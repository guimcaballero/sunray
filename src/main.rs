use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
use hittable::*;
mod hittable_list;
use hittable_list::*;

fn main() {
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
}

fn get_image_string() -> String {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u8;

    // World
    let mut world = HittableList::new();
    let sphere = Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let big_sphere = Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    world.add(&sphere);
    world.add(&big_sphere);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    let mut result = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        // println!("Remaining scanlines: {}", j);
        for i in 0..image_width {
            let u = (i as f32) / (image_width - 1) as f32;
            let v = (j as f32) / (image_height - 1) as f32;
            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let color = ray_color(&ray, &world);

            color.write_color(&mut result);
        }
    }

    return result;
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_record = HitRecord::default();
    if world.hit(&ray, 0.0, f32::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.3, 0.3, 1.0)
}
