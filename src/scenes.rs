use crate::{camera::*, hittable::*, hittable_list::*, material::*, perlin::*, texture, vec3::*};
use rand::Rng;

#[allow(dead_code)]
pub enum Scene {
    ManySpheres,
    TwoPerlinSpheres,
}

pub fn generate_world(scene: Scene, aspect_ratio: f64) -> (HittableList, Camera) {
    match scene {
        Scene::ManySpheres => many_spheres(aspect_ratio),
        Scene::TwoPerlinSpheres => two_perlin_spheres(aspect_ratio),
    }
}

fn many_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    // Ground
    let checker = texture::checker(
        texture::solid_color(Color::new(0.2, 0.3, 0.1)),
        texture::solid_color(Color::new(0.9, 0.9, 0.9)),
    );
    world.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(checker),
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
        material: Material::Lambertian(texture::noise(Perlin::new(), 4.0)),
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
        material: Material::Lambertian(texture::solid_color(Color::new(2.0, 2.0, 1.0))),
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
                        material: Material::Lambertian(texture::solid_color(albedo)),
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

    (world, camera)
}

fn two_perlin_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let texture = texture::marble(Perlin::new(), 4.0);
    world.add(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(1.0, 1.0, 1.0), 0.0),
    }));
    world.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(texture.clone()),
    }));

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let vfov = 20.0;
    let aperture = 0.0;

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

    (world, camera)
}
